use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String,
    password: String,
    tasks: HashMap<u32, Task>,
    next_task_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoApp {
    users: HashMap<String, User>,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    fn add_task(&mut self, username: &str, title: String, description: String) {
        if let Some(user) = self.users.get_mut(username) {
            let task = Task {
                id: user.next_task_id,
                title,
                description,
                completed: false,
            };
            user.tasks.insert(user.next_task_id, task);
            user.next_task_id += 1;
            self.save_to_file();
        } else {
            println!("Користувача не знайдено");
        }
    }

    fn delete_task(&mut self, username: &str, task_id: u32) {
        if let Some(user) = self.users.get_mut(username) {
            user.tasks.remove(&task_id);
            self.save_to_file();
        } else {
            println!("Користувача не знайдено");
        }
    }

    fn edit_task(&mut self, username: &str, task_id: u32, new_title: String, new_description: String) {
        if let Some(user) = self.users.get_mut(username) {
            if let Some(task) = user.tasks.get_mut(&task_id) {
                task.title = new_title;
                task.description = new_description;
                self.save_to_file();
            } else {
                println!("Завдання не знайдено");
            }
        } else {
            println!("Користувача не знайдено");
        }
    }

    fn mark_task_completed(&mut self, username: &str, task_id: u32) {
        if let Some(user) = self.users.get_mut(username) {
            if let Some(task) = user.tasks.get_mut(&task_id) {
                if task.completed {
                    task.completed = false;
                }
                else {
                    task.completed = true;
                }
                self.save_to_file();
            } else {
                println!("Завдання не знайдено");
            }
        } else {
            println!("Користувача не знайдено");
        }
    }

    fn show_all_tasks(&self, username: &str) {
        if let Some(user) = self.users.get(username) {
            for task in user.tasks.values() {
                if task.completed{
                    println!(
                        "ID: {}, Назва: {}, Опис: {}, Завершено: так",
                        task.id, task.title, task.description
                    );
                }else {
                    println!(
                        "ID: {}, Назва: {}, Опис: {}, Завершено: ні",
                        task.id, task.title, task.description
                    );
                }
            }
        } else {
            println!("Користувача не знайдено");
        }
    }

    fn save_to_file(&self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = File::create("todo_data.json").unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn load_from_file(filename: &str) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if contents.is_empty() {
            Self::new()
        } else {
            serde_json::from_str(&contents).unwrap()
        }
    }

    fn register_user(&mut self, username: String, password: String) {
        if self.users.contains_key(&username) {
            println!("Користувач уже існує");
        } else {
            let user = User {
                username: username.clone(),
                password,
                tasks: HashMap::new(),
                next_task_id: 1
            };
            self.users.insert(username, user);
            self.save_to_file();
            println!("Зареєстровано успішно");
        }
    }

    fn authenticate_user(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            user.password == password
        } else {
            false
        }
    }
}

fn main() {
    let mut app = TodoApp::load_from_file("todo_data.json");
    let mut logged_in_user = String::new();

    loop {
        if logged_in_user.is_empty() {
            println!("Виберіть дію: \n1) Реєстрація \n2) Увійти \n3) Вийти");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => {
                    println!("Введіть ім'я користувача для реєстрації:");
                    let mut username = String::new();
                    io::stdin().read_line(&mut username).unwrap();
                    let username = username.trim().to_string();

                    println!("Введіть пароль:");
                    let mut password = String::new();
                    io::stdin().read_line(&mut password).unwrap();
                    let password = password.trim().to_string();

                    app.register_user(username, password);
                },
                "2" => {
                    println!("Введіть ім'я користувача:");
                    let mut username = String::new();
                    io::stdin().read_line(&mut username).unwrap();
                    let username = username.trim().to_string();

                    println!("Введіть пароль:");
                    let mut password = String::new();
                    io::stdin().read_line(&mut password).unwrap();
                    let password = password.trim().to_string();

                    if app.authenticate_user(&username, &password) {
                        println!("Вхід успішний.");
                        logged_in_user = username;
                    } else {
                        println!("Невірне ім'я користувача або пароль.");
                    }
                },
                "3" => break,
                _ => println!("Невірний вибір."),
            }
        } else {
            println!("\nВиберіть дію: \n1) Додати завдання \n2) Видалити завдання \
            \n3) Редагувати завдання \n4) Позначити як виконане \n5) Показати всі завдання \
            \n6) Вийти з аккаунту");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            match choice {
                "1" => {
                    println!("Введіть назву завдання:");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).unwrap();

                    println!("Введіть опис завдання:");
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).unwrap();

                    app.add_task(&logged_in_user, title.trim().to_string(), description.trim().to_string());
                },
                "2" => {
                    println!("Введіть ID завдання для видалення:");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();
                    if let Ok(id) = id.trim().parse::<u32>() {
                        app.delete_task(&logged_in_user, id);
                    } else {
                        println!("Невірний формат ID.");
                    }
                },
                "3" => {
                    println!("Введіть ID завдання для редагування:");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();
                    if let Ok(id) = id.trim().parse::<u32>() {
                        println!("Введіть нову назву завдання:");
                        let mut new_title = String::new();
                        io::stdin().read_line(&mut new_title).unwrap();

                        println!("Введіть новий опис завдання:");
                        let mut new_description = String::new();
                        io::stdin().read_line(&mut new_description).unwrap();

                        app.edit_task(&logged_in_user, id, new_title.trim().to_string(), new_description.trim().to_string());
                    } else {
                        println!("Невірний формат ID.");
                    }
                },
                "4" => {
                    println!("Введіть ID завдання для позначення як виконане:");
                    let mut id = String::new();
                    io::stdin().read_line(&mut id).unwrap();
                    if let Ok(id) = id.trim().parse::<u32>() {
                        app.mark_task_completed(&logged_in_user, id);
                    } else {
                        println!("Невірний формат ID.");
                    }
                },
                "5" => {
                    app.show_all_tasks(&logged_in_user);
                },
                "6" => {
                    logged_in_user.clear();
                    println!("Ви вийшли з акаунту.");
                },
                _ => println!("Невірний вибір."),
            }
        }
    }
}

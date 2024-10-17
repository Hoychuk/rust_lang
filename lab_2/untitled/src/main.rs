use std::collections::VecDeque;
use std::io;

// Функція для пріоритету операторів
fn get_precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2, // Високий пріоритет
        '(' => 0, // Низький пріоритет для відкритих дужок
        _ => 0,

    }
}

// Перевірка, чи символ є оператором
fn is_operator(c: char) -> bool {
    c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')'
}

// Перетворення інфіксного виразу у зворотну польську нотацію
fn infix_to_rpn(expression: &str) -> Result<VecDeque<String>, &'static str> {
    let mut output: VecDeque<String> = VecDeque::new();
    let mut operators: Vec<char> = Vec::new();
    let mut number = String::new();

    for ch in expression.chars() {
        if ch.is_digit(10) || ch == '.' {
            number.push(ch); // Будуємо число
        } else if ch == '(' {
            operators.push(ch);
        } else if ch == ')' {
            if !number.is_empty() {
                output.push_back(number.clone());
                number.clear();
            }
            // Виймаємо всі оператори до відкритої дужки
            while let Some(op) = operators.pop() {
                if op == '(' {
                    break;
                }
                output.push_back(op.to_string());
            }
        } else if is_operator(ch) {
            if !number.is_empty() {
                output.push_back(number.clone());
                number.clear();
            }
            while !operators.is_empty() && get_precedence(*operators.last().unwrap()) >= get_precedence(ch) {
                output.push_back(operators.pop().unwrap().to_string());
            }
            operators.push(ch);
        } else {
            return Err("Помилка. Невірний символ у виразі");
        }
    }

    if !number.is_empty() {
        output.push_back(number);
    }

    while let Some(op) = operators.pop() {
        if op == '(' || op == ')' {
            return Err("Помилка. Невірні дужки");
        }
        output.push_back(op.to_string());
    }

    Ok(output)
}

// Обчислення виразу в зворотній польській нотації
fn evaluate_rpn(rpn: VecDeque<String>) -> Result<f64, &'static str> {
    let mut stack: Vec<f64> = Vec::new();

    for token in rpn {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else if token.len() == 1 {
            let operator = token.chars().next().unwrap();
            if stack.len() < 2 { //Нема чисел у стеку перед дією
                return Err("Помилка. Неправильний вираз");
            }
            let operand2 = stack.pop().unwrap();
            let operand1 = stack.pop().unwrap();
            let result = match operator {
                '+' => operand1 + operand2,
                '-' => operand1 - operand2,
                '*' => operand1 * operand2,
                '/' => {
                    if operand2 == 0.0 {
                        return Err("Помилка. Ділення на нуль");
                    }
                    operand1 / operand2
                }
                _ => return Err("Помилка. Невірний оператор"),
            };
            stack.push(result);
        } else {
            return Err("Помилка. Неправльний токен");
        }
    }

    if stack.len() == 1 {
        Ok(stack.pop().unwrap())
    } else {
        Err("Помилка. Неправильний вираз")
    }
}

fn main() {
    let mut memory: Option<f64> = None; // Пам'ять для збереження результату

    loop {
        println!("\nУведіть вираз, 'exit' - вийти, 'mem' - збережений результат:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не вдалось прочитати");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input == "mem" {
            if let Some(result) = memory {
                println!("Збережений результат: {}", result);
            } else {
                println!("Немає збереженого результата");
            }
            continue;
        }

        match infix_to_rpn(input) {
            Ok(rpn) => {
                match evaluate_rpn(rpn) {
                    Ok(result) => {
                        println!("Результат: {}", result);
                        memory = Some(result); // Збереження результату
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

use std::io;

fn main() {
    println!("Calculate!");

    let first_num: f64 = loop {
        println!("What's the first number?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the string");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please use a valid number!"),
        }
    };

    let second_num: f64 = loop {
        println!("What's the second number?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the string");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please use a valid number!"),
        }
    };

    let operator = loop {
        println!("What's the operator?");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the string");

        match input.trim() {
            "+" | "-" | "*" | "/" => break input.trim().to_string(),
            _ => println!("Please use a valid operator!"),
        }
    };

    match operator.as_str() {
        "+" => {
            println!("Result: {} {} {} = {:.10}", first_num, operator, second_num, first_num + second_num);
        },
        "-" => {
            println!("Result: {} {} {} = {:.10}", first_num, operator, second_num, first_num - second_num);
        },
        "*" => {
            println!("Result: {} {} {} = {:.10}", first_num, operator, second_num, first_num * second_num);
        },
        "/" => {
            println!("Result: {} {} {} = {:.10}", first_num, operator, second_num, first_num / second_num);
        },
        _ => println!("Invalid operator"),
    }
}

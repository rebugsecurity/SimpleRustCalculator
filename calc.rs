use std::io;

fn main() {
    loop {
        println!("Enter your equation: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let result = calculate(&input);

        match result {
            Ok(value) => println!("Result: {}", value),
            Err(msg) => eprintln!("Error: {}", msg),
        }
    }
}

fn calculate(input: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = input.trim().split(' ').collect();

    if tokens.len() != 3 {
        return Err("Invalid input".into());
    }

    let left: f64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid input".into()),
    };

    let right: f64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid input".into()),
    };

    match tokens[1] {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" if right == 0.0 => Err("Division by zero".into()),
        "/" => Ok(left / right),
        _ => Err("Invalid operator".into()),
    }
}
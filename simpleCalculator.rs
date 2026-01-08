use std::io;

fn main() {
    println!("Simple calculator (enter 'exit' to quit)");
    loop {
        // ask for the first number
        println!("Enter the first integer :");

        let mut first_input = String::new();
        io::stdin().read_line(&mut first_input).expect("Failed to read line");
        let trimmed_first = first_input.trim();

        // Check for exit condition
        if trimmed_first.eq_ignore_ascii_case("exit") {
            println!("Bye bye! Exiting the clalculator!");
            break;
        }

        // Parse the first integer
        let num1: f64 = match trimmed_first.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                continue;
            }
            
        };

        // ask for the operand
        println!("Enter the operand (+, -, *, /):");

        let mut operator_input = String::new();
        io::stdin().read_line(&mut operator_input).expect("Failed to read line");
        let operator = operator_input.trim();
        
        
        // Print the result if the operand is ² or ^10
        // Perform calculation
        match operator { 
            "²" => {
                let temp_result = num1 * num1;
                println!("²: {}", temp_result);
            },
            "^10" => {
                let temp_result = num1 * 10.0;
                println!("^10: {}", temp_result);
            },
            _ => {
                    // continue;
                    // ask for the second number
                    println!("Enter the second integer:");

                    let mut second_input = String::new();
                    io::stdin().read_line(&mut second_input).expect("Failed to read line");
                    let trimmed_second = second_input.trim();

                    // parse the second integer
                    let num2: f64 = match trimmed_second.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Invalid input. Please enter a valid integer.");
                            continue;
                        }
                    };

                    // Perform calculation
                    let result = match operator {
                        "+" => num1 + num2,
                        "-" => num1 - num2,
                        "*" => num1 * num2,
                        "/" => {
                            if num2 == 0.0 {
                                println!("Error: Division by zero.");
                                continue;
                            }
                            num1 / num2
                        }
                        "%" => num1 % num2,
                        _ => {
                            println!("Unsupported operator: {}", operator);
                            continue;
                        }
                    };
                    
                    println!("Result: {} {} {} = {}\n\n\n", num1, operator, num2, result);
            }
        }
        
    };
        
        
        
        
        
        
        
        

        
    
}

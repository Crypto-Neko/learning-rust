use std::io::{self, Write}; // Import methods for use in the program

// Take in floats x and y and apply op to them
fn calculate(x: f32, y: f32, op: &str) -> f32 {
    // Determine what op is and return the resulting value
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => {
            if y != 0.0 {   // Check that y is not 0
                x / y
            }
            // Return NAN if division by 0 is attempted
            else {
                println!("Error: Division by zero");
                f32::NAN
            }
        },
        _ => {  // Returns NAN and prints an error if the operation is entered incorrectly
            println!("Invalid operation");
            f32::NAN
        }
    }

}

fn main() {
    // Define the string to contain the input
    let mut input = String::new();
    
    // Get the value of x
    print!("Enter x: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let x: f32 = input.trim().parse().expect("Invalid number.");
    input.clear();

    //  Get the value of y
    print!("Enter y: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let y: f32 = input.trim().parse().expect("Invalid number.");
    input.clear();

    // Get the operation
    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let op = input.trim();

    // Compute and print the result of the calculation
    let result = calculate(x, y, op);
    if !result.is_nan() {
        println!("{} {} {} = {}", x, op, y, result);
    }
}

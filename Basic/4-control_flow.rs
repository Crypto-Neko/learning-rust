use std::io::{self, Write}; // Import methods for use in the program

fn main() {
    // Define the string to contain the input
    let mut input = String::new();

    // Get the number
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let num: f32 = input.trim().parse().expect("Invalid number.");
    
    // Determine whether num is positive or negative
    if num > 0.0 {
        println!("{} > 0", num);
    }
    else if num < 0.0 {
        println!("{} < 0", num);
    }
    else if num == 0.0 {
        println!("{} = 0", num);
    }
}

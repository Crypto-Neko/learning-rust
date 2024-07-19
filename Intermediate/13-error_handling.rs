use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter an integer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let int: i32 = input.trim().parse().expect("Not an integer.");
    println!("The integer ({}) is valid.", int);
}

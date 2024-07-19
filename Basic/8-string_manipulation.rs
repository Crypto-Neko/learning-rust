fn main() {
    let message: &str = "Hello there";
    println!("{}", message);
    
    let concat_message = format!("{}.", message);
    println!("{}", concat_message);

    let rev_message: String = message.chars().rev().collect();
    println!("{}", rev_message);
}

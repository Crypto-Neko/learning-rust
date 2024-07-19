struct Example {
    example_data: String,
}

impl Example{
    fn ownership_demo(self) {
        println!("{}", self.example_data);
    }
}

fn main() {
    let example = Example { example_data: String::from("example of ownership") };
    example.ownership_demo();

    // Attempting to use example after it has been moved
    // This line will cause a compile-time error
    println!("After method call: {}", example.example_data);
}

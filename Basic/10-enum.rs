#[allow(dead_code)]
enum PolComp {
    AuthLeft,
    AuthRight,
    LibRight,
    LibLeft,
}

fn main() {
    let my_quad = PolComp::LibRight;

    match my_quad {
        PolComp::AuthLeft => println!("Commie!"),
        PolComp::AuthRight => println!("Fascist!"),
        PolComp::LibRight => println!("Fascist but with corporations!"),
        PolComp::LibLeft => println!("Fascist but with wokeness...also hippie!"),
    }
}

    

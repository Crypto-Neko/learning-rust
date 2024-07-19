fn fibonacci(n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut a = 1;
    let mut b = 1;
    let mut temp;

    result.push(a);
    result.push(b);

    let mut remaining = n;
    while remaining > 0 {
        result.push(a + b);
        temp = a;
        a = b;
        b = b + temp;
        remaining -= 1;
    }

    result
}

fn main() {
    let fib = fibonacci(10);
    println!("{:?}", fib);
}

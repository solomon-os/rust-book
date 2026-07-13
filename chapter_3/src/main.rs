fn main() {
    let difference = 95.5 / 43.2;

    println!("difference:  {difference}");

    let quotient = -9 / 2;

    println!("quotien: {quotient}");
    let (a, b) = another_function();

    println!("tuple: {a}, {b}");

    let fibonacci_no = fibonacci(10);

    println!("6th fibonacci number {fibonacci_no}")
}

fn another_function() -> (i32, u128) {
    (0, 999)
}

fn fibonacci(x: u32) -> u32 {
    let mut prev: u32 = 0;
    let mut current: u32 = 1;

    let mut counter = 1;

    while counter < x {
        let tmp = current + prev;
        prev = current;
        current = tmp;
        counter += 1;
    }
    current
}

fn main() {
    let difference = 95.5 / 43.2;

    println!("difference:  {difference}");

    let quotient = -9 / 2;

    println!("quotien: {quotient}");
    let (a, b) = another_function();

    println!("tuple: {a}, {b}");
}

fn another_function() -> (i32, u128) {
    return (0, 999);
}

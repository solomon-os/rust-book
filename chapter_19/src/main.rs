enum Message {
    Hello { id: i32 },
}

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let message = Message::Hello { id: 5 };

    match message {
        Message::Hello { id: id @ 1..=5 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 6..=10 } => {
            println!("Found an id in another range")
        }
        _ => {
            println!("Default ran")
        }
    }
}

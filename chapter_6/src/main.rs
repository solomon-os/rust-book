enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = value_in_cents(Coin::Penny);
    println!("coin value in cents {coin}");

    let dice_roll = 9;

    match dice_roll {
        3 => println!("3"),
        7 => println!("7"),
        other => println!("dice_roll: {other}"),
    }

    let Some(optional_value) = optional_value() else {
        return;
    };

    println!("optional value: {optional_value}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn optional_value() -> Option<i32> {
    Some(33)
}

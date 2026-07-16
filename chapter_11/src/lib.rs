fn greeting(greet: &str) -> &str {
    return greet;
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.")
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.")
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn greeting_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {result}"
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

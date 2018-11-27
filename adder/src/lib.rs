pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        self.length < other.length && self.width >= other.width
    }
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}", name)
    String::from("yay")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!(
        //         "Value should be bigger than 1 and below 100. Got {}.",
        //         value
        //     );
        // }

        if value < 1 {
            panic!("Value should be lower than 100. Got {}.", value);
        } else if value > 100 {
            panic!("Value should be bigger than 1. Got {}.", value);
        }
        Guess { value: value }
    }
}

pub fn print_and_returns(num: i32) -> i32 {
    println!("I got the value {}", num);
    10
}

pub fn prints_and_returns_10(num: i32) -> i32 {
    println!("We got the value {}", num);
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Something unexpected happened!");
    }

    #[test]
    #[ignore]
    fn larger_can_hold() {
        let larger = Rectangle {
            length: 10,
            width: 10,
        };
        let smaller = Rectangle {
            length: 5,
            width: 5,
        };
        assert!(larger.can_hold(smaller));
    }

    #[test]
    #[ignore]
    fn smaller_cannot_hold() {
        let larger = Rectangle {
            length: 10,
            width: 10,
        };
        let smaller = Rectangle {
            length: 5,
            width: 5,
        };
        assert!(!smaller.can_hold(larger));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(name);

        assert!(
            result.contains("Carol"),
            "Greeting didn't contain {}, instead returned {}",
            name,
            result
        );
    }

    #[test]
    #[should_panic(expected = "Value should be lower than 100.")]
    #[ignore]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two doesn't equal four."))
        }
    }

    #[test]
    #[ignore]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);

        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

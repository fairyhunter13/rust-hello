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
    num + 3
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}", name)
    String::from("yay")
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    fn test_add_two() {
        assert_eq!(4, add_two(2));
    }

    // #[test]
    fn another() {
        panic!("Something unexpected happened!");
    }

    // #[test]
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

    // #[test]
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
}

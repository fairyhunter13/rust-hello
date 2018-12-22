use std::{
    fmt::{Display, Formatter, Result},
    ops::Add,
};

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// Another example: specifying other type for Add trait.
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking!");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Waving arms furiously!");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using supertrait.
// A trait that depends on another trait.
// Like, adding trait bound to a trait.
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// Using the Newtype pattern.
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    println!("Hello, world!");

    // assert_eq!(
    //     Point { x: 1, y: 2 } + Point { x: 1, y: 2 },
    //     Point { x: 2, y: 4 }
    // );

    let person = Human {};
    person.fly();
    // Calling specifically method from implemented trait
    Pilot::fly(&person);
    Wizard::fly(&person);

    // Calling associated function need to be full qualified syntax.
    // We need to call the implemented animal trait. The code below return "Spot".
    // What we expect is "puppy".
    println!("A baby dog is called a {}.", Dog::baby_name());

    // Rust doesn't know which implementation to use.
    // println!("A baby dog is called a {}", Animal::baby_name());

    // Using fully qualified syntax instead.
    println!("A baby dog is called a {}.", <Dog as Animal>::baby_name());

    let point = Point { x: 1, y: 2 };
    point.outline_print();

    // Let's try our Newtype pattern.
    let w = Wrapper(vec!["hello".to_owned(), "world".to_owned()]);
    println!("w = {}", w);
}

// Trait std::ops::Add
// trait Add<RHS=Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

// Specifying the default parameter for a trait can help for:
// 1. To extend a type without breaking existing code
// 2. To allow customization in specific cases most users won’t need

// Those cases is useful when we want to overload the operators (+,-, etc) behavior.

// Calling methods with the same name (disambiguation)

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

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

fn main() {
    println!("Hello, world!");

    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 1, y: 2 },
        Point { x: 2, y: 4 }
    );
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

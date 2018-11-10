#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let rect = (30, 50);
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rect 2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect 3? {}", rect.can_hold(&rect3));

    println!("The rect is {:#?}", rect);

    println!("The new square is {:#?}", Rectangle::square(5));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

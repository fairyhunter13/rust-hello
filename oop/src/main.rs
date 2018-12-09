extern crate oop;

use oop::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 15,
                height: 10,
                options: vec!["Yes".to_owned(), "Maybe".to_owned(), "No".to_owned()],
            }),
            Box::new(Button {
                width: 5,
                height: 10,
                label: "Ok".to_owned(),
            }),
            // This code below won't compile, because String doesn't implement Draw Trait.
            // Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}

use std::ops::Deref;
use List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    let k = *b + 1;
    println!("b = {}", b);
    println!("k = {}", k);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let aisatsu = MyBox::new(String::from("Rust"));
    hello(&aisatsu);

    // Type without deref coercien really hard to read.
    // Deref coercien happens at compile time.
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my_stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other_stuff"),
    };

    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created!");

    //Explicit destructor cannot be called directly.
    // c.drop();

    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

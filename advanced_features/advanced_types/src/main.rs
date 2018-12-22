use std::fmt::{self, Arguments};
use std::io::Error;

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y= {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}

// These two function takes a long generic parameter type.
// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
//     // --snip--
// }

fn takes_long_type(f: Thunk) {
    // --snip--
}

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
//     // --snip--
// }

// fn returns_long_type() -> Thunk {
//     // --snip--
// }

// Considering this Write trait example.
// The pattern of "Result<.., Error>" is very common.
// Instead, we can make an alias for the "Result".
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

// type Result<T> = Result<T, Error>;

// The syntax right now is much simpler and nice.
// You can also still using the propagation syntax (?) or another method that works for Result<T, Error>.
// The type alias makes code easier and consistent interface.

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//     fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
// }

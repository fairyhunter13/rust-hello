use std::fmt::{self, Arguments};
use std::io::Error;

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y= {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    print!("forever ");

    // Forever loop expression can also be inferred as never return.
    // Except, if there is a break statement.
    loop {
        print!("and ever ");
    }
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

// Diverging function: return never function.
// Never in diverging function is annotated as "!".
// There are no concrete values regarding this never "!".

// fn bar() -> ! {
//     // --snip--
// }

// For Example:
// let guess: u32 = match guess.trim().parse() {
//     Ok(num) => num,
//     Err(_) => continue,
// };
// Match arm can't possibly have two types of values.
// So, it's only just one type.
// But, it can infer the "!", so it will pick type other than never.
// Code below doesn't compiled.
// let guess = match guess.trim().parse() {
//     Ok(_) => 5,
//     Err(_) => "hello",
// }

// Another example of return  never with panic:
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
// }
//     }

// DST or dynamically sized types
// Types that have dynamic size and unknown at compile time.
// Generic function in rust always added a special trait (Sized) by rust automatically.
// For example:

fn generic<T>(t: T) {
    // --snip--
}

// Could be treated as this by Rust:
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }

// Relax the restriction:
// fn generic<T: ?Sized>(t: &T) {
//     // --snip--
// }

// str is also treated as DST, because we don't know how much memory string will take at compile time.
// We use some kind of pointer or smart pointer here, to refer the address and the length of str.
// We use can use &, Box, Rc.
// This code below won't compile:
// let test: str = "Hello, test!";

extern crate advanced_lifetimes;

use advanced_lifetimes::{Ball, Red};

fn main() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

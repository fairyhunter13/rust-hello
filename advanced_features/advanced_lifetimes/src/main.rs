extern crate advanced_lifetimes;

use advanced_lifetimes::{Ball, BallTest, Red};

static NUM: i32 = 5;

fn main() {
    let num = 5;
    // The lifetime for obj in here is equal to 'a defined
    // from the Ball type that implementing Red trait.
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;

    let obj_1 = Box::new(BallTest { diameter: &NUM }) as Box<dyn Red>;
}

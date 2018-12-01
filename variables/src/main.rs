const MAX_POINTS: u32 = 100_000;

fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    //Rust doesn't let you mutate the variable type.
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;

    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 3 * 40;
    let quotient = 56.7 / 72.2;
    let remainder = 43 % 5;

    let flag = true;

    let otherFlag: bool = false;

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    // println!("The value of x is {}", x);
    // println!("The value of y is {}", y);
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // Index out of bounds section!
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}

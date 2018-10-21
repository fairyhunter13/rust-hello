fn main() {
    println!("Hello, world!");

    // another_function(5, 6);
    let x = five();
    println!("x is equal to: {}", x);

    let y = x_plus_one(3);

    println!("y is equal to: {}", y)
}

fn another_function(num1: i32, num2: i32) {
    println!("Another Function!");
    println!("The value of num1 and num2 is: {} and {}", num1, num2);
}

fn five() -> i32 {
    5
}

fn x_plus_one(x: i32) -> i32 {
    x + 1
}

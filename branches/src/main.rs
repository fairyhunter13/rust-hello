fn main() {
    println!("Hello, world!");
    let number = 6;
    if number < 5 {
        println!("Number is lower than 5!");
    } else {
        println!("Number is equal or higher than 5!");
    }
    if number % 4 == 0 {
        println!("Number is divisible by 4!");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3!");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2!");
    } else {
        println!("Number is not divisible by 2,3,4!");
    }
}

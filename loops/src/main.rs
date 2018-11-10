fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 {
        println!("The number now is: {}", number);

        number -= 1;
    }
    println!("Done decrementing the number!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    println!("Using while: ");
    while index < 5 {
        println!("The current element is {}", a[index]);
        index += 1;
    }
    println!("Using for in: ");
    for elem in a.iter() {
        println!("The current element is {}", elem);
    }
    println!("Using for loop in range!");
    for elem in (1..4).rev() {
        println!("The current elem is {}", elem);
    }
}

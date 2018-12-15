fn main() {
    let favorite_color: Option<&str> = None;

    let is_tuesday = false;

    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple");
        } else {
            println!("Using orange");
        }
    } else {
        println!("Using blue");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    // Printing in LIFO order.
    while let Some(top) = stack.pop() {
        println!("The last elem is: {}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("The elem in index: {} and vlaue: {}", index, value);
    }

    // let declarations
    let x = 5;
    let (x, y) = (1, 2);
    // Ignoring other values
    let (x, ..) = (1, 2, 3, 4);
    // Can't be used in here.
    // The compiler doesn't know which element we specify in here.
    // let (..,x,..) = (1, 2, 3, 4);

    let point = (3, 5);
    print_coordinates(&point);

    // Refutable and Irrefutable Patterns
    // Irrefutable Patterns: let, for, and function params.
    // Refutable Patterns: if let, while let.
    let some_value: Option<i32> = None;

    // Attempting to use refutable pattern in irrefutable situation.
    // let Some(x) = some_value;

    if let Some(x) = some_value {
        println!("{}", x);
    }
    // If let can't be used on irrefutable patterns.
    // if let x = 5 {
    //     println!("{}", x);
    // };
}

// Function Declaration
fn foo(x: i32) {}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("The coordinates of x: {} and y: {}", x, y);
}

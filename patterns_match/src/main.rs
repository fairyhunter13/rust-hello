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

    // Pattern Syntax
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Any numbers!"),
    }

    //  Block inside match pattern is a new scope.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Using or in pattern matching
    let x = 2;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // Using range pattern
    let x = 5;

    // Exclusive range pattern is for experimental only.
    // Using numeric value.
    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Using range in chars.
    let x = 'c';

    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring structs
    // let p = Point { x: 5, y: 7 };

    // let Point { x: a, y: b } = p;

    // assert_eq!(5, a);
    // assert_eq!(7, b);

    // let p = Point { x: 5, y: 7 };

    // let Point { x, y } = p;

    // assert_eq!(5, x);
    // assert_eq!(7, y);

    // let p = Point { x: 0, y: 7 };

    // match p {
    //     Point { x, y: 0 } => println!("On the x axis at {}", x),
    //     Point { x: 0, y } => println!("On the y axis at {}", y),
    //     Point { x, y } => println!("On neither axis: x:{} & y:{}", x, y),
    // }

    // Destructuring Enums
    // let message = Message::ChangeColor(0, 160, 255);

    // match message {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to restructure.");
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {} and in the y direction {}", x, y);
    //     }
    //     Message::Write(text) => {
    //         println!("The message is {}", text);
    //     }
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the r: {}, g: {}, b: {}", r, g, b);
    //     }
    // }

    // Destructuring nested enums & structs

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Hsv(x, y, z)) => {
            println!(
                "Change the color to hue {}, saturation {}, value {}",
                x, y, z
            );
        }

        Message::ChangeColor(Color::Rgb(x, y, z)) => {
            println!("Change the color to red {}, green {}, blue {}", x, y, z);
        }
        _ => (),
    }

    // Destructuring References
    // let points = vec![
    //     Point { x: 0, y: 0 },
    //     Point { x: 1, y: 5 },
    //     Point { x: 10, y: -3 },
    // ];

    // let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    // println!("The sum of squares is: {}", sum_of_squares);

    // Destructuring complex tuple and structs
    // let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring values in pattern.
    // Using .., _, and underscore with a name.
    foo(5, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value!");
        }

        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("Setting is {:#?}", setting_value);

    let numbers = (2, 4, 6, 8, 16);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers in {}, {}, {}", first, third, fifth);
        }
    }

    // Using underscore in prefix of variable name
    let _x = 5;
    let y = 6;

    println!("The number y is {}", y);

    let s = Some(String::from("Hello!"));

    // This code below still binds the value of s to _s.
    // So, the code below move ownership of s to _s.
    // if let Some(_s) = s {
    //     println!("found a string");
    // }

    // This code below doesn't bind the s value to _.
    // Because it doesn't move the ownership.

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point { x: 5, y: 6, z: 7 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 6, 8, 10, 12);

    match numbers {
        (first, .., last) => println!("The first: {} & last: {}", first, last),
    }

    // Using .. in match pattern can be ambigous.
    // Using with careful please.
    // This code below won't compile because it is ambious.
    //  let numbers = (2, 4, 8, 16, 32);

    //     match numbers {
    //         (.., second, ..) => {
    //             println!("Some numbers: {}", second)
    //         },
    //     }

    // Using match guards.
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five. x: {}", x),
        Some(x) => println!("x: {}", x),
        _ => (),
    }

    // Solving shadowed variable using match guard.

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(x) if x == y => println!("Matched x: {}", x),
        _ => println!("Default case. x: {:#?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let msg = Test::Hello { id: 5 };

    match msg {
        Test::Hello {
            id: id_variable @ 3...7,
        } => println!("Found an id in range: {}", id_variable),
        Test::Hello { id: 10...12 } => println!("Found an id in another range."),
        Test::Hello { id } => println!("Found some other id: {}", id),
    }
}

enum Test {
    Hello { id: i32 },
}

fn foo(_: i32, y: i32) {
    println!("This code only use y param: {}", y);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// Function Declaration
// fn foo(x: i32) {}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("The coordinates of x: {} and y: {}", x, y);
}

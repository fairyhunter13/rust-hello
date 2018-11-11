// enum IpAddrKind {
//     V4,
//     V6,
// }

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

#[derive(Debug, Copy, Clone)]
struct EventOne {
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone)]
struct EventTwo {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
enum ComplexEvent {
    A(EventOne, i32),
    B(EventTwo, i32),
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!(
            "This is the value of enum: {:#?}",
            match self {
                Message::Write(text) => text,
                Message::Quit => "test",
                Message::Move { x, y } => "Move msg",
                Message::ChangeColor(x, y, z) => "Change Color message",
            }
        );
    }
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    println!("Hello, world!");

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let loopback = IpAddrKind::V6(String::from("::1"));

    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello enum!"));
    m.call();

    let x = ComplexEvent::A(EventOne { x: 0.1, y: 0.1 }, 5);
    // match x {
    //     ComplexEvent::A(EventOne { x, y }, s) => println!("{}, {}, {}", x, y, s),
    //     ComplexEvent::B(EventTwo { x, y }, s) => println!("{}, {}, {}", x, y, s),
    // }

    let test = EventOne { x: 0.1, y: 0.1 };
    let k = test;
    let z = x;

    println!("The x is {:#?}", x);

    println!("The k is {:#?}", k);
    println!("The z is {:#?}", z);

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    println!("Check value if have moved {:#?}", five);

    let some_value = Some(0u8);
    if let Some(3) = some_value {
        println!("Three!");
    }
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("The count: {:?}!", count);
    }
}

fn route(_ipAddr: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}

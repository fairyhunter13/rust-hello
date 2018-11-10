fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("The value of s1: {} s2: {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);

    let y = 5;
    makes_copy(y);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_give_back(s2);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The string {} has size: {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    change(&mut s);

    println!("The mutable s is {}", s);

    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn change(text: &mut String) {
    text.push_str(" shit");
}

fn calculate_length(text: &String) -> usize {
    text.len()
}

fn takes_and_give_back(text: String) -> String {
    text
}

fn gives_ownership() -> String {
    let some_string = String::from("test");
    some_string
}

fn takes_ownership(text: String) {
    println!("THe text is {}", text);
}

fn makes_copy(integer: i32) {
    println!("The integer is {}", integer);
}

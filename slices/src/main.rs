fn main() {
    println!("Hello, world!");
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    let slice = &s[0..len];
    let slice = &s[..];

    let mut s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("The word is {}", word);

    let string_literal = "Hello from the other world!";
    let word = first_word(string_literal);

    println!("The literal world is {}", word);
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..index];
        }
    }
    &text[..]
}

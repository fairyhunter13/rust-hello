use std::collections::HashMap;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v = vec![1, 2, 3, 4, 5];
    let test = &v[2];
    let v_index = 2;

    match v.get(v_index) {
        Some(_) => println!("Reachable element at index: {}", v_index),
        None => println!("Unreachable element at index: {}", v_index),
    }

    // let x = &v[100];
    let x = v.get(100);
    // println!("This is {:#?}", v)
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("This is i: {}", i);
    }
    let mut v = vec![100, 32, 57];
    {
        let third_element = &mut v[2];
        *third_element = 5;
    }
    for i in &mut v {
        *i += 1;
    }
    println!("This is vector v: {:#?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row 1 of vector row is: {:#?}", row[0]);
    println!("Vector is: {:#?}", row);

    let mut s = String::new();

    let initial_data = "Hello from the me!";
    let initial_s = initial_data.to_string();

    let initial_s = "Some content!".to_string();
    println!("The initial data is {}", initial_data);

    let initial_s = String::from("using from!");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    let mut bar = String::from("bar");
    s.push_str(&mut bar);
    bar.push_str("test");
    bar.push('l');
    println!("s is {}", s);
    println!("bar becomes {}", bar);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("Print the rest string! {} && {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("This is string slice: {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    println!("========================");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    println!("========================");

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // println!("The field name is {}", field_name);

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let field_name = "Blue";
    let score = scores.get(field_name);

    for (key, value) in &scores {
        println!("Key {}: Value: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Blue", 25);

    println!("Scores: {:#?}", scores);

    let mut scores = HashMap::new();
    scores.insert("Blue", 10);

    let mut y = 100;
    scores.entry("Blue").or_insert(50);
    {
        let test = scores.entry("Yellow").or_insert(50);
    }
    println!("Scores: {:#?}", scores);

    let text = "Hello world, guys!";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("The map now is: {:#?}", map);
}

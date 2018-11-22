use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("Hello world!");

    // let v = vec![9, 1, 2, 34];
    // v[99];

    let f: Result<_, _> = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Failed to create hello.txt. Error: {:#?}", error),
    //         },
    //         other_error => panic!("Another unexpected error! Error: {:#?}", other_error),
    //     },
    // };

    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error in creating the file: {:#?}", error);
            })
        } else {
            panic!("There was problem in opening the file! Error: {:#?}", error)
        }
    });

    // let f = File::open("hello.txt").unwrap();
    //Using this is much simple and easier.
    /*Test multiple comment*/
    let f = File::open("hello.txt").expect(&"This is an error opening hello.txt");
}

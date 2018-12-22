fn add_two(x: i32) -> i32 {
    x + 2
}

// Function Pointer
// Function pointer is used to pass a regular function to another function as arguments.
// Function pointer is labeled as "fn" (lowercase).

// Function pointer is a type not a trait.
// So, there is no generic parameter like closures for trait bounds.
// Function pointer implement all closure's traits, Fn, FnOnce, and FnMut.

// Best practice is to define function with closures as arguments and closure's trait as trait bounds.
// So the function can accept either function or closure.

// Implement function pointer only happens in cases when interfacing with C.
// C doesn't have closures.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_two, 2);

    println!("The answer is {}", answer);

    // Cases where function can accept either closure or function.
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    // Using fully qualified syntax because there are many to_string methods.
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let closure = returns_closure();
}

// Closures don't have any concrete type that is returnable.
// Function pointer doesn't work too with closure for return type.
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }
// This is happening because the code above state that closures are Different DST.
// So, we must use pointer or smart pointer in here.
// Since, closure can be a different type and DST, rust recommend us to use trait object.
// Trait object has another name such as dynamic object.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

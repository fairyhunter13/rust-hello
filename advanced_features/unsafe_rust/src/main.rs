use std::slice;

// Define global variable.
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;
    // Making mutable and immutable raw pointers.
    // * is part of the name in row pointers.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Using arbitrary memory.
    let address = 0x012345usize;
    let r = address as *const i32;

    // Requires unsafe to dereference raw pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    // Encapsulate unsafe implementation inside safe abstraction.

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    // let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Using raw poitner to make a slice with arbitrary memory.
    // When we use the slice, undefined behavior could occur.
    // For example, segmentation fault.
    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // println!("Test with trying the slice: {:#?}", slice);

    // Using external function could be dangerous.
    // Need to encapsulate it inside the unsafe scope.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    // Example of using static mutable.
    // Read and write of static mutable need to use unsafe keyword.
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Unsafe function declaration
unsafe fn dangerous() {}

// Example of split_at_mut as a function instead of a method.
// Code below won't compile because two mutable borrows occur inside the same scope.
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }

// Using raw pointer below might help!
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        println!("The first element: {:?}", *ptr);
        println!("The second element: {:?}", *ptr.offset(mid as isize));
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

// Try to use C standard library through C's ABI.
extern "C" {
    fn abs(input: i32) -> i32;
}

// We could export function inside rust too.
// We don't need to use unsafe keyword in here.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Using static mutable need to use unsafe keyword.
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Declaring and implementing unsafe trait.
// A trait is unsafe when at least one of its
// methods has some invariant that the compiler can’t verify.

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};
use List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// Multiple ownership with mutable values.
// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

//Beware of reference cycles.
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // let b = Box::new(5);
    // let k = *b + 1;
    // println!("b = {}", b);
    // println!("k = {}", k);

    // // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let x = 5;
    // let y = &x;
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let aisatsu = MyBox::new(String::from("Rust"));
    // hello(&aisatsu);

    // Type without deref coercien really hard to read.
    // Deref coercien happens at compile time.
    // let m = MyBox::new(String::from("Rust"));
    // hello(&(*m)[..]);

    // let c = CustomSmartPointer {
    //     data: String::from("my_stuff"),
    // };

    // let d = CustomSmartPointer {
    //     data: String::from("other_stuff"),
    // };

    // println!("CustomSmartPointers created.");

    // let c = CustomSmartPointer {
    //     data: String::from("some data"),
    // };

    // println!("CustomSmartPointer created!");

    // //Explicit destructor cannot be called directly.
    // // c.drop();

    // drop(c);

    // println!("CustomSmartPointer dropped before the end of main.");

    // This won't work because c and b have multiple ownership to a.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // We do this instead.
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Rc::new(Cons(3, Rc::clone(&a)));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Rc::new(Cons(4, Rc::clone(&a)));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c is out of scope = {}", Rc::strong_count(&a));

    // Using Rc and RefCell combined to get multiple ownership and mutable values.
    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    // Reference Cycles Examples (Beware this is dangerous)
    // Rust cannot prevent this.
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("Leaf parent is : {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!(
        "Leaf parent after assignment is : {:?}",
        leaf.parent.borrow().upgrade()
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}

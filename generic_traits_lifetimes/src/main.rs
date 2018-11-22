struct Point<T, U> {
    x: T,
    y: U,
}

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let num_largest = largest(&number_list);

    println!("The largest number is {}", num_largest);

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65, 170, 182];

    let num_largest = largest(&number_list);

    println!("The largest number is {}", num_largest);

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);

    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);

    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 4 };
    let float = Point { x: 5.0, y: 1.0 };

    // let multi_generic = Point { x: 5.0, y: 4 };

    let p = Point { x: 3, y: 5 };

    println!("X is {}", p.x);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: "c" };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.into_iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

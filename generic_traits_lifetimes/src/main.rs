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

// trait Summary {
//     fn summarize(&self) -> String;
// }

// trait Summary {
//     fn summarize_author(&self) -> String {};
//     fn summarize(&self) -> String {
//         "Read more...".to_owned()
//     }
// }

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}.", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Test Summarize Author")
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
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

    let tweet = Tweet {
        username: "horse".to_owned(),
        content: "This is tweet from horse".to_owned(),
        reply: false,
        retweet: false,
    };

    println!("This is the summarize of tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Penguins wins the cup!".to_owned(),
        location: "Indonesia".to_owned(),
        author: "Iceburgh".to_owned(),
        content: "The Pittsburg Penguins are the best!".to_owned(),
    };

    println!("This is the new article: {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
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

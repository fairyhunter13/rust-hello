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

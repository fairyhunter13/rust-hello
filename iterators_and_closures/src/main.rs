extern crate iterators_and_closures;

use iterators_and_closures::{Cacher, Counter};
use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 2;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("item: {}", val);
    }

    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    for num in Counter::new().skip(1) {
        println!("Test skip: {}", num);
    }
}

// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Today do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hidrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_results = simulated_expensive_calculation(intensity);
//     if intensity < 25 {
//         println!("Today do {} pushups!", expensive_results);
//         println!("Today do {} situps!", expensive_results);
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hidrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_results);
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly!");
//         thread::sleep(Duration::from_secs(2));
//         intensity
//     };
//     if intensity < 25 {
//         println!("Today do {} pushups!", expensive_closure(intensity));
//         println!("Today do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hidrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_results = Cacher::new(|num: u32| -> u32 {
        println!("calculating slowly!");
        thread::sleep(Duration::from_secs(2));
        intensity
    });
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_results.value(intensity));
        println!("Today do {} situps!", expensive_results.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hidrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_results.value(intensity)
            );
        }
    }
}

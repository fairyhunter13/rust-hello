use std::{thread, time::Duration};

fn main() {
    // Wait second thread until finish before main thread exits.
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hi, number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("Hi, number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // Wait spawned thread finish, then main thread continue to work.
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi, number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

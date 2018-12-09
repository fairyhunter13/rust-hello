use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

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

    // Using move closures with threads.

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("The vector is {:?}", v);
    });

    handle.join().unwrap();

    //First Example
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // The val that has been sent inside the tx cannot be used again
        // because the val has been moved. (ownership transfer)
        // println!("The val is: {}", val);
    });

    let received = rx.recv().unwrap();

    println!("Got {}", received);

    // Second example.

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            "hi".to_owned(),
            "hello".to_owned(),
            "this is channel sending".to_owned(),
            "Nice to meet you!".to_owned(),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got string: {}", received);
    }

    // Using multiple producers concept!
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            "hi".to_owned(),
            "from".to_owned(),
            "the".to_owned(),
            "thread".to_owned(),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got [Multiple Producers]: {}", received);
    }

    // Using mutual exclusion in rust.

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("M is {:?}", m);

    // Using mutual exclusion in multiple threads.

    // let counter = Mutex::new(0);

    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    // Code above is not compiled yet, because counter variable is moved.
    // Remember the ownership system.

    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();

    //     *num += 1;
    // });
    // handles.push(handle);

    // let handle2 = thread::spawn(move || {
    //     let mut num2 = counter.lock().unwrap();

    //     *num2 += 1;
    // });
    // handles.push(handle2);

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());

    // Using Rc now like in multiple ownership.

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

extern crate hello;

use hello::ThreadPool;
use std::{net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // thread::spawn(|| {
        //     if let Err(err) = hello::handle_connections(stream) {
        //         println!("Handle connection error: {}", err);
        //     }
        // });

        pool.execute(|| {
            if let Err(err) = hello::handle_connections(stream) {
                println!("Handle connection error: {}", err);
            }
        });
    }

    println!("Shutting down!");
}

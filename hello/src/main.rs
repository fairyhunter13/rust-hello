extern crate final_project;

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if let Err(err) = hello::handle_connections(stream) {
            println!("Handle connection error: {}", err);
        }
    }
}

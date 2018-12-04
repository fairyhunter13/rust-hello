extern crate minigrep;

use minigrep::*;
use std::{env, process};

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> (Config) {
//     Config {
//         query: args[1].clone(),
//         filename: args[2].clone(),
//     }
// }

use std::{env, fs, process};

struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:#?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("args: {:#?}", args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    run(config);
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.filename).expect("Something wrong when reading the file!");

    println!("With text:\n {}", contents);
}

// fn parse_config(args: &[String]) -> (Config) {
//     Config {
//         query: args[1].clone(),
//         filename: args[2].clone(),
//     }
// }

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        Ok(Config {
            query: args[1].to_owned(),
            filename: args[2].to_owned(),
        })
    }
}

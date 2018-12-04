use std::{
    env::{self, Args},
    error::Error,
    fs,
};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &str> {
    //     if args.len() < 3 {
    //         return Err("Not enough arguments!");
    //     }

    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    //     Ok(Config {
    //         query: args[1].to_owned(),
    //         filename: args[2].to_owned(),
    //         case_sensitive: case_sensitive,
    //     })
    // }

    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get the query string!"),
        };

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a filename!"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents =
    //     fs::read_to_string(config.filename).expect("Something wrong when reading the file!");
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("The line is: {}", line)
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// Improved version using iterator

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Version 1: pub fn new(args: &[String]) -> Result<Config, &str> {

    // Updating the signature of Config::new to expect an iterator
    // the env::args function shows that the type of the iterator it returns is std::env::Args
    // we’re taking ownership of args and we’ll be mutating args by iterating over it
    // With the change to args, the lifetime elision rules no longer apply, and we must specify the 'static lifetime.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        /**
        Version 1:
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        **/

        // Changing the body of Config::new to use iterator methods

        //  the first value in the return value of env::args is the name of the program.
        // We want to ignore that and get to the next value
        args.next();

        // If next returns a Some, we use a match to extract the value.
        // If it returns None, it means not enough arguments were given and we return early with an Err value.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // ? will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /**
    Version 1:
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
    **/

    // Using iterator adaptor methods in the implementation of the search function
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// CASE_INSENSITIVE=1 cargo run to poem.txt
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /**
    Version 1:
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
    **/

    // Using iterator adaptor methods in the implementation of the search function
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
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
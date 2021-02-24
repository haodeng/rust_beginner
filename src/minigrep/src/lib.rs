use std::env;
use std::error::Error;
use std::fs;
/**
We’ve made liberal use of the pub keyword: on Config, on its fields and its new method, and on the run function.
We now have a library crate that has a public API that we can test!
**/

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // a function named new that is associated with the Config struct.
    // Making this change will make the code more idiomatic.
    // We can create instances of types in the standard library, such as String, by calling String::new.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // returns a Result with a Config instance in the success case and a &str in the error case.
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // The env::var function returns a Result
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// returned the unit type, (), and we keep that as the value returned in the Ok case.
// For the error type, we used the trait object Box<dyn Error>
// Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
// This gives us flexibility to return error values that may be of different types in different error cases.
// The dyn keyword is short for “dynamic.”
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

// lifetime 'a used with the contents argument and the return value.
// the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
// In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
// n other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// CASE_INSENSITIVE=1 cargo run to poem.txt
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
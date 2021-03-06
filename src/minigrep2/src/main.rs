// Improved version using iterator

use std::env;
use std::process;
use minigrep2::Config;

fn main() {
    /**
     Version 1:
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    **/

    // The env::args function returns an iterator!
    // Rather than collecting the iterator values into a vector (in Version 1) and then passing a slice to Config::new
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does.
    // Because run returns () in the success case, we only care about detecting an error,
    // so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
    // The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.
    if let Err(e) = minigrep2::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

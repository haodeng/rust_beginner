// bring the std::env module into scope with a use statement so we can use its args function.
use std::env;
use std::process;

// bring the Config type from the library crate into the binary crate’s scope
use minigrep::Config;

fn main() {
    // cargo run needle haystack
    // ["target/debug/minigrep", "needle", "haystack"]
    // the first value [0] in the vector is "target/debug/minigrep", which is the name of our binary.
    // This matches the behavior of the arguments list in C
    let args: Vec<String> = env::args().collect();

    // Using unwrap_or_else allows us to define some custom, non-panic! error handling.
    // If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping.
    // However, if the value is an Err value, this method calls the code in the closure,
    // which is an anonymous function we define and pass as an argument to unwrap_or_else.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln! macro that prints to the standard error stream
        eprintln!("Problem parsing arguments: {}", err);
        // stop the program immediately and return the number that was passed as the exit status code.
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does.
    // Because run returns () in the success case, we only care about detecting an error,
    // so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
    // The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;

fn main() {
    // open_file_only();
    // matching_errors();
    // matching_errors_closure();
    // unwrap();
    // expect();

    let result = propagating_error();
    let result = match result {
        Ok(String) => String,
        Err(error) => panic!("{:?}", error),
    };
    println!("{}", result);

    let result = propagating_error_shortcut();
    let result = match result {
        Ok(String) => String,
        Err(error) => panic!("{:?}", error),
    };
    println!("{}", result);

    let result = propagating_error_chain_shortcut();
    let result = match result {
        Ok(String) => String,
        Err(error) => panic!("{:?}", error),
    };
    println!("{}", result);

    let result = propagating_error_concise_version();
    let result = match result {
        Ok(String) => String,
        Err(error) => panic!("{:?}", error),
    };
    println!("{}", result);
}

fn open_file_only() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


fn matching_errors() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn matching_errors_closure() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// If we run this code without a hello.txt file, we’ll see an error message from the panic! call that the unwrap method makes
fn unwrap() {
    let f = File::open("hello.txt").unwrap();
}

// We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
// The error message used by expect in its call to panic! will be the parameter that we pass to expect,
// rather than the default panic! message that unwrap uses
fn expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Propagating Errors
// When you’re writing a function whose implementation calls something that might fail,
// instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do
fn propagating_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn propagating_error_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chaining method calls after the ? operator
fn propagating_error_chain_shortcut() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Reading a file into a string is a fairly common operation,
// so Rust provides the convenient fs::read_to_string function that opens the file,
// creates a new String, reads the contents of the file, puts the contents into that String, and returns it.
fn propagating_error_concise_version() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
# Error Handling
Rust groups errors into two major categories: recoverable and unrecoverable errors. 
For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. 
Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.


Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. 
Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. 

## Unrecoverable Errors with panic!
By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting in your Cargo.toml file.

    [profile.release]
    panic = 'abort'

Using panic! macro. Error: thread 'main' panicked at 'crash and burn', src/main.rs:2:5

    fn main() {
        panic!("crash and burn");
    }

Using a panic! Backtrace. Error: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    
    fn main() {
        let v = vec![1, 2, 3];
        v[99];
    }
 
 A backtrace is a list of all the functions that have been called to get to this point. Similiar to stacktrace in java.
 Get a backtrace by setting the RUST_BACKTRACE environment variable to any value except 0
 
     RUST_BACKTRACE=1 cargo run
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99',     /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
    stack backtrace:
       0: backtrace::backtrace::libunwind::trace
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
       1: backtrace::backtrace::trace_unsynchronized
       .....
   
   In order to get backtraces with this information, debug symbols must be enabled. 
   Debug symbols are enabled by default when using cargo build or cargo run without the --release flag
  
    
## Recoverable Errors with Result
Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.

the Result enum is defined as having two variants, Ok and Err, as follows:

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

T represents the type of the value that will be returned in a success case within the Ok variant, and E represents the type of the error that will be returned in a failure case within the Err variant.

    use std::fs::File;

    fn main() {
        // return type of the File::open function is a Result<T, E>
        // T has been filled in here with the type of the success value, std::fs::File, which is a file handle. 
        // The type of E used in the error value is std::io::Error.
        let f = File::open("hello.txt");
        
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }
    

## Matching on Different Errors

    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
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

That’s a lot of match! The match expression is very useful but also very much a primitive.
Improved version. Result<T, E> type has many methods that accept a closure and are implemented using match expressions.

    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
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

    
    

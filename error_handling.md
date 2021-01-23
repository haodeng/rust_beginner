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

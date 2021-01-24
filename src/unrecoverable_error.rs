fn main() {
    // Using panic! marco
    // panic!("crash and burn");

    panic_with_backtrace();
}

// Using a panic! Backtrace
// To get a backtrace, run the program by setting the RUST_BACKTRACE environment variable to any value except 0,
// for example: RUST_BACKTRACE=1 cargo run
// or: RUST_BACKTRACE=1 ./unrecoverable_error
fn panic_with_backtrace() {
    let v = vec![1, 2, 3];

    v[99];
}
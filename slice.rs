fn main() {
    let s = String::from("hello world");

    // [starting_index..ending_index]
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}.", hello, world);

    // They are equal. start at the first index (zero)
    let slice1 = &s[0..2];
    let slice2 = &s[..2];
    println!("{} {}.", slice1, slice2);

    // They are equal. slice includes the last byte of the String
    let len = s.len();
    let slice1 = &s[3..len];
    let slice2 = &s[3..];
    println!("{} {}.", slice1, slice2);

    // They are equal. take a slice of the entire string
    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("{} {}.", slice1, slice2);


    let mut s = String::from("hello world");
    let word = first_word(&s);
    //  if we have an immutable reference to something, we cannot also take a mutable reference
    // s.clear(); // error! error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    // first_word_slice_param works on slices of `String`s
    let word = first_word_slice_param(&my_string[..]);
    println!("{}.", word);

    let my_string_literal = "hello world";
    // first_word_slice_param works on slices of string literals
    let word = first_word_slice_param(&my_string_literal[..]);
    println!("{}.", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_param(my_string_literal);
    println!("{}.", word);
}

// return a slice. The type that signifies “string slice” is written as &str
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// it allows us to use the same function on both &String values and &str values.
// If we have a String, we can pass a slice of the entire String.
// Defining a function to take a string slice instead of a reference to a String makes our API more general
// and useful without losing any functionality
fn first_word_slice_param(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
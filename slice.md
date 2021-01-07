# String Slices
We can create slices using a range within brackets by specifying [starting_index..ending_index]

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    
They are equal
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
    
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    
    let slice = &s[0..len];
    let slice = &s[..];

The type that signifies “string slice” is written as &str:

    fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

        &s[..]
    }

## String Literals Are Slices

    // The type of s here is &str: it’s a slice pointing to that specific point of the binary. 
    // This is also why string literals are immutable; &str is an immutable reference.
    let s = "Hello, world!";


## String Slices as Parameters
it allows us to use the same function on both &String values and &str values.

    fn first_word(s: &str) -> &str {

If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String. 
Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionalit.

    fn main() {
        let my_string = String::from("hello world");

        // first_word works on slices of `String`s
        let word = first_word(&my_string[..]);

        let my_string_literal = "hello world";

        // first_word works on slices of string literals
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
    }

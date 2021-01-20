# String
The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

## Creating a New String

    // creates a new empty string called s, which we can then load data into. 
    let mut s = String::new();
    
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    
    // is equivalent to the code that uses to_string
    let s = String::from("initial contents");

## Updating a String
push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter. 

    let mut s = String::from("foo");
    s.push_str("bar");
    
    // Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');

## Concatenation with the + Operator or the format! Macro

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

we used a reference to s2 has to do with the signature of the method that gets called when we use the + operator. The + operator uses the add method, whose signature looks something like this:

    fn add(self, s: &str) -> String {

But wait—the type of &s2 is &String, not &str. 
The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]. 


If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

For more complicated string combining, we can use the format! macro:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

## Indexing into Strings
If you try to access parts of a String using indexing syntax in Rust, you’ll get an error.

    let s1 = String::from("hello");
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    let h = s1[0];

    let hello = "Здравствуйте";
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    let answer = &hello[0];

Rust strings don’t support indexing. 

## Slicing Strings
you can use [] with a range to create a string slice containing particular bytes

    let hello = "Здравствуйте";
    let s = &hello[0..4];
s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.
What would happen if we used &hello[0..1]? The answer: Rust would panic at runtime
You should use ranges to create string slices with caution, because doing so can crash your program.

## Methods for Iterating Over Strings

    // If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the chars method.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    // The bytes method returns each raw byte, which might be appropriate for your domain:
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    
    

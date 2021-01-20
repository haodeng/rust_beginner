fn main() {
    // creates a new empty string called s, which we can then load data into
    let mut s = String::new();
    s.push_str("hello");
    println!("{}", s);

    let data = "initial contents";
    println!("{}", data);
    let s = data.to_string();
    println!("{}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("{}", s);

    // String::from to create a String from a string literal.
    // The code is equivalent to the code that uses to_string.
    let s = String::from("initial contents");

    valid_strings();

    let mut s = String::from("foo");
    // push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter
    s.push_str("bar");
    println!("{}", s);

    // Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // used a reference to s2 has to do with the signature of the method that gets called when we use the + operator.
    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // With all of the + and " characters, it’s difficult to see what’s going on.
    // Use format! macro is better
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // For more complicated string combining, we can use the format! macro
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // If you try to access parts of a String using indexing syntax in Rust, you’ll get an error.
    // Rust strings don’t support indexing.
    let s1 = String::from("hello");
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // let h = s1[0];

    let hello = "Здравствуйте";
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // let answer = &hello[0];

    // Slicing Strings is ok
    // use [] with a range to create a string slice containing particular bytes
    let hello = "Здравствуйте";
    // s will be a &str that contains the first 4 bytes of the string.
    // each of these characters was 2 bytes, which means s will be Зд.
    let s = &hello[0..4];
    println!("{}", s);

    // Rust would panic at runtime
    // thread 'main' panicked at 'byte index 1 is not a char boundary;
    // it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/libcore/str/mod.rs:2069:5
    // let s = &hello[0..1];
    // You should use ranges to create string slices with caution, because doing so can crash your program.


    iterate_string();
}

fn iterate_string() {
    // If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the chars method.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // The bytes method returns each raw byte, which might be appropriate for your domain
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    for c in "你好".chars() {
        println!("{}", c);
    }
    for b in "你好".bytes() {
        println!("{}", b);
    }
}

// strings are UTF-8 encoded. All of these are valid String values.
fn valid_strings() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
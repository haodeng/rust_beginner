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
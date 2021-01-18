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

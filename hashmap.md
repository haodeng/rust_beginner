# Creating a New Hash Map
hash maps store their data on the heap. This HashMap has keys of type String and values of type i32. 
Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

Another way of constructing a hash map is by using iterators and the collect method on a vector of tuples, where each tuple consists of a key and its value. 

    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
 
 The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures and Rust doesn’t know which you want unless you specify. 
 For the parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.


# Hash Maps and Ownership
For types that implement the Copy trait, like i32, the values are copied into the hash map. 
For owned values like String, the values will be moved and the hash map will be the owner of those values

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.

# Accessing Values in a Hash Map

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    
    // iterate over each key/value pair in a hash map 
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
 Here, score will have the value that’s associated with the Blue team, and the result will be Some(&10). 
 The result is wrapped in Some because get returns an Option<&V>; if there’s no value for that key in the hash map, get will return None
 
 # Updating a Hash Map
 ## Overwriting a Value
 
     use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

## Only Inserting a Value If the Key Has No Value

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, 
and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. 

## Updating a Value Based on the Old Value

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    
The or_insert method actually returns a mutable reference (&mut V) to the value for this key. 
Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). 
The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

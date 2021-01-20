use std::collections::HashMap;

fn main() {
    create_hashmaps();

    hashmap_ownership();

    access_value_in_hashmap();

    updating_a_hashmap();
}

fn create_hashmaps() {
    // hash maps store their data on the heap.
    // This HashMap has keys of type String and values of type i32.
    // hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // Another way of constructing a hash map is by using iterators and the collect method on a vector of tuples,
    // where each tuple consists of a key and its value.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // HashMap<_, _> is needed here because it’s possible to collect into many different data structures
    // and Rust doesn’t know which you want unless you specify.
    // For the parameters for the key and value types, however, we use underscores,
    // and Rust can infer the types that the hash map contains based on the types of the data in the vectors.
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
}

fn hashmap_ownership() {
    // We aren’t able to use the variables field_name and field_value
    // after they’ve been moved into the hash map with the call to insert.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn access_value_in_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // the result will be Some(&10).
    // The result is wrapped in Some because get returns an Option<&V>;
    // if there’s no value for that key in the hash map, get will return None.
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // iterate over each key/value pair in a hash map
    for (key, value) in &scores {
        // will print each pair in an arbitrary order
        println!("{}: {}", key, value);
    }
}

fn updating_a_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // Overwriting a Value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Only Inserting a Value If the Key Has No Value
    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
    // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // Updating a Value Based on the Old Value
    for word in text.split_whitespace() {
        // The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
        // Here we store that mutable reference in the count variable,
        // so in order to assign to that value, we must first dereference count using the asterisk (*).
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
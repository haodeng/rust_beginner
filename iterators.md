# Processing a Series of Items with Iterators
In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

    let v1 = vec![1, 2, 3];

    // created an iterator. no iteration takes place here
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

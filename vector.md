# Creating a New Vector
Vec<T>, also known as a vector. We added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.

    let v: Vec<i32> = Vec::new();

It’s more common to create a Vec<T> that has initial values, and Rust provides the vec! macro for convenience. 

    // Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary.
    let v = vec![1, 2, 3];


# Updating a Vector

    // if we want to be able to change its value, we need to make it mutable using the mut keyword
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.

# Dropping a Vector Drops Its Elements
When the vector gets dropped, all of its contents are also dropped, meaning those integers it holds will be cleaned up.

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here


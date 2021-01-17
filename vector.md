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


# Reading Elements of Vectors
There are two ways to reference a value stored in a vector.
First, we use the index value of 2 to get the third element: vectors are indexed by number, starting at zero. Second, the two ways to get the third element are by using & and [], which gives us a reference, or by using the get method with the index passed as an argument, which gives us an Option<&T>

    let v = vec![1, 2, 3, 4, 5];

    // gives us a reference
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // gives us an Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    //Wrong! cause the program to panic because it references a nonexistent element.
    let does_not_exist = &v[100];
    //Ok. it returns None without panicking. 
    let does_not_exist = v.get(100);


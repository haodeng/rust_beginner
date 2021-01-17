fn main() {

    // hold elements of the i32 type
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // It’s more common to create a Vec<T> that has initial values, and Rust provides the vec! macro for convenience.
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // Won't work. vectors can only store values that are the same type.
    // let v3 = vec![1, "2", 3.0001];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    // When the vector gets dropped, all of its contents are also dropped,
    // meaning those integers it holds will be cleaned up.
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    // The two ways to get the third element are:
    // by using & and [], which gives us a reference
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // or by using the get method with the index passed as an argument, which gives us an Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // will cause the program to panic because it references a nonexistent element.
    //let does_not_exist = &v[100];

    // get is ok, return None
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);

    // push
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    println!("The six element is: {}", &v[5]);

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value that the mutable reference refers to,
        // we have to use the dereference operator (*) to get to the value in i
        *i += 50;
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // vectors can only store values that are the same type. This can be inconvenient;
    // the variants of an enum are defined under the same enum type,
    // so when we need to store elements of a different type in a vector, we can define and use an enum!
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
fn main() {
    let int8: i8 = 1;
    let uint8: u8 = 1;
    println!("i8: {}, u8: {}", int8, uint8);

    let x = 2.0; // f64, default type
    println!("The value of x is: {}", x);

    let y: f32 = 3.0; // f32
    println!("The value of y is: {}", y);

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}",difference);

    // multiplication
    let product = 4 * 30;
    println!("{}",product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}",quotient);

    // remainder
    let remainder = 43 % 5;
    println!("{}",remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{}, {}",t, f);

    //Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
    // which means it can represent a lot more than just ASCII
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}",c, z, heart_eyed_cat);

    let tup = (500, 6.4, 1);
    //To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    //n addition to destructuring through pattern matching,
    // we can access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let tup: (i32, f64, u8) = (500, 6.4, 1); //optional type annotations
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    //This is the same as writing let a = [3, 3, 3, 3, 3];
    let a = [3; 5];
    println!("{}", a[0]);
}


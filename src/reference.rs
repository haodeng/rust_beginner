fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //So what happens if we try to modify something we’re borrowing? it doesn’t work!
    // let s = String::from("hello");
    // change(&s);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}.", s);

    // mutable references have one big restriction:
    // you can have only one mutable reference to a particular piece of data in a particular scop
    let mut s2 = String::from("hello");
    let r1 = &mut s2;
    // let r2 = &mut s2; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("{}.", r1);

    //cannot have a mutable reference while we have an immutable one
    let mut s3 = String::from("hello");
    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    // let r3 = &mut s3; // BIG PROBLEM, error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);

    let mut s4 = String::from("hello");
    let r1 = &s4; // no problem
    let r2 = &s4; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s4; // no problem
    println!("{}", r3);

    //My test
    let mut s5 = String::from("hello");
    let r5 = &mut s5;
    r5.push_str(", world");
    println!("{}", r5);
    // println!("{}, {}", s5, r5); // error[E0502]: cannot borrow `s5` as immutable because it is also borrowed as mutable

    let s6 = String::from("hello");
    let r6 = &s6;
    println!("{}, {}", s6, r6);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

// `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
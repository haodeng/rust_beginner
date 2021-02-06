
// error[E0597]: `x` does not live long enough
// fn not_compile() {
//     {
//         let r;
//
//         {
//             let x = 5;
//             // borrowed value does not live long enough
//             r = &x;
//         }
//
//         println!("r: {}", r);
//     }
// }

// error[E0106]: missing lifetime specifier
// expected named lifetime parameter
// help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// fn longest_not_compile(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime 'a.
// The function signature also tells Rust that the string slice returned from the function will live
// at least as long as lifetime 'a.
// In practice, it means that the lifetime of the reference returned by the longest function
// is the same as the smaller of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// we’ve specified a lifetime parameter 'a for the parameter x and the return type,
// but not for the parameter y,
// because the lifetime of y does not have any relationship with the lifetime of x or the return value.
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// even though we’ve specified a lifetime parameter 'a for the return type,
// this implementation will fail to compile because the return value lifetime
// is not related to the lifetime of the parameters at all.
// fn longest_not_compile2<'a>(x: &str, y: &str) -> &'a str {
    // error[E0515]: cannot return value referencing local variable `result`
//     let result = String::from("really long string");
//     result.as_str()
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}



fn main() {
    // not_compile();

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.level());
    println!("{}", i.announce_and_return_part("hi"));

    // The Static Lifetime
    // 'static, which means that this reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}
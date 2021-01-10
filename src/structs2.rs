// print out debugging information
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of Rectangle, we start an impl (implementation) block.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions, don’t take self as a parameter.
    // They’re functions, NOT methods, because they don’t have an instance of the struct to work with.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // an immutable borrow to rect2, an instance of Rectangle.
    // This makes sense because we only need to read rect2 (rather than write,
    // which would mean we’d need a mutable borrow),
    // and we want main to retain ownership of rect2 so we can use it again after calling the can_hold method.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is {:?}", rect1); // print debug info. `{:?}` (or {:#?} for pretty-print)

    println!(
        "The area of the rectangle is {} square pixels.",
        // Rust has a feature called automatic referencing and dereferencing.
        // when you call a method with object.something(),
        // Rust automatically adds in &, &mut, or * so object matches the signature of the method.
        // In other words, the following are the same:
        // p1.distance(&p2);
        // (&p1).distance(&p2);
        rect1.area()
    );

    // Call Associated Functions: Rectangle::square
    println!("Square is {:?}", Rectangle::square(10));
}
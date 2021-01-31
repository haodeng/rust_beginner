
// This won't work
// For now, this error states that the body of largest won’t work for all possible types that T could be.
// Because we want to compare values of type T in the body, we can only use types whose values can be ordered.
// To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // error[E0369]: binary operation `>` cannot be applied to type `T`
        // note: `T` might need a bound for `std::cmp::PartialOrd`
        // if item > largest {
        //     largest = item;
        // }
    }

    largest
}

// the Point<T> struct is generic over some type T,
// and the fields x and y are both that same type, whatever that type may be.
// If we create an instance of a Point<T> that has values of different types,  our code won’t compile.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
// we have to declare T just after impl so we can use it to specify that we’re implementing methods on the type Point<T>.
// By declaring T as a generic type after impl, Rust can identify
// that the type in the angle brackets in Point is a generic type rather than a concrete type.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
// the type Point<f32> will have a method named distance_from_origin
// and other instances of Point<T> where T is not of type f32 will not have this method defined.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// To define a Point struct where x and y are both generics but could have different types,
// we can use multiple generic type parameters.
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);

    println!("p.x = {}", integer.x());
    println!("distance_from_origin = {}", float.distance_from_origin());

    // error[E0599]: no method named `distance_from_origin` found for struct `Point<{integer}>` in the current scope
    //println!("distance_from_origin = {}", integer.distance_from_origin());

    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("{:?}", integer_and_float);
}
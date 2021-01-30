# Generic Data Types
You can use any identifier as a type parameter name. But we’ll use T because, by convention, parameter names in Rust are short, often just a letter, 
and Rust’s type-naming convention is CamelCase. 
Short for “type,” T is the default choice of most Rust programmers.

## In Function Definitions

      // We read this definition as: the function largest is generic over some type T. 
      // This function has one parameter named list, which is a slice of values of type T. 
      // The largest function will return a reference to a value of the same type T.
      fn largest<T>(list: &[T]) -> &T {
      
  
This won't work

    fn largest<T>(list: &[T]) -> &T {
      let mut largest = &list[0];

      for item in list {
            // error[E0369]: binary operation `>` cannot be applied to type `T`
            if item > largest {
                  largest = item;
            }
      }
      largest
      }
      
this error states that the body of largest won’t work for all possible types that T could be. 
Because we want to compare values of type T in the body, we can only use types whose values can be ordered. 
To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types

## In Struct Definitions
this definition says that the Point<T> struct is generic over some type T, and the fields x and y are both that same type, whatever that type may be. 
If we create an instance of a Point<T> that has values of different types, our code won’t compile.
      
      struct Point<T> {
            x: T,
            y: T,
      }

      fn main() {
            let integer = Point { x: 5, y: 10 };
            let float = Point { x: 1.0, y: 4.0 };
            
            // error[E0308]: mismatched types
            // let wont_work = Point { x: 5, y: 4.0 };
      }

To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters. 
we can change the definition of Point to be generic over types T and U where x is of type T and y is of type U.

    struct Point<T, U> {
      x: T,
      y: U,
    }
    
    fn main() {
      let both_integer = Point { x: 5, y: 10 };
      let both_float = Point { x: 1.0, y: 4.0 };
      let integer_and_float = Point { x: 5, y: 4.0 };
    }

## In Enum Definitions
generic over type T and has two variants: Some, which holds one value of type T, and a None variant that doesn’t hold any value.

    enum Option<T> {
      Some(T),
      None,
    }

Enums can use multiple generic types as well. The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and Err, which holds a value of type E.

    enum Result<T, E> {
      Ok(T),
      Err(E),
    }

## In Method Definitions
We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions,
Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
      
    struct Point<T> {
      x: T,
      y: T,
    }

    // T just after impl so we can use it to specify that we’re implementing methods on the type Point<T>. 
    // By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
    impl<T> Point<T> {
      fn x(&self) -> &T {
        &self.x
      }
    }
    
    // We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type. 
    // we use the concrete type f32, meaning we don’t declare any types after impl.
    // This code means the type Point<f32> will have a method named distance_from_origin 
    // and other instances of Point<T> where T is not of type f32 will not have this method defined.
    impl Point<f32> {
      fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
      }
    }
    

    fn main() {
      let p = Point { x: 5, y: 10 };
      println!("p.x = {}", p.x());
    }

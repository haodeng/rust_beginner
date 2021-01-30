# Generic Data Types
You can use any identifier as a type parameter name. But we’ll use T because, by convention, parameter names in Rust are short, often just a letter, 
and Rust’s type-naming convention is CamelCase. 
Short for “type,” T is the default choice of most Rust programmers.

      // We read this definition as: the function largest is generic over some type T. 
      // This function has one parameter named list, which is a slice of values of type T. 
      // The largest function will return a reference to a value of the same type T.
      fn largest<T>(list: &[T]) -> &T {
      
  

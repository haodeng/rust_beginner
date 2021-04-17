# Smart Pointers
A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data. 
They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of pointer we use most often.

Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities.

Smart pointers are usually implemented using structs. 
The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits. 
The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers. 
The Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

## Using Box<T> to Point to Data on the Heap
Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

* When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
* When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

### Using a Box<T> to Store Data on the Heap
We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap. 
  
  fn main() {
      let b = Box::new(5);
      println!("b = {}", b);
  }
  

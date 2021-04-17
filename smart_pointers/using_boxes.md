# Using Box<T> to Point to Data on the Heap
Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

* When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
* When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

## Using a Box<T> to Store Data on the Heap
We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap. 
  
    fn main() {
        let b = Box::new(5);
        println!("b = {}", b);
    }

Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).

Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in this way very often. Having values like a single i32 on the stack, where they’re stored by default, is more appropriate in the majority of situations. 
  

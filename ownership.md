# Stack and Heap
Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways. 
The stack stores values in the order it gets them and removes the values in the opposite order. 
This is referred to as last in, first out.

All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. 
The heap is less organized.
This process is called allocating on the heap and is sometimes abbreviated as just allocating.

Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. 
Allocating a large amount of space on the heap can also take time.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, 
and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

All primitive data types are on the stack, String is on heap.


# Ownership rules
* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.


# Memory and Allocation
Why can String be mutated but literals cannot? The difference is how these two types deal with memory.

    let mut s = String::from("hello"); // This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
    
    let s2 = "hello"; // immutable, can not "mut" it

In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. 
This is why string literals are fast and efficient. 
But these properties only come from the string literal’s immutability. 

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

* The memory must be requested from the memory allocator at runtime. String::from, its implementation requests the memory it needs.
* We need a way of returning this memory to the allocator when we’re done with our String.


Some language use GC, some have to manually allocate and deallocate it. Rust takes a different approach: the memory is automatically returned once the variable that owns it goes out of scope. 
When a variable goes out of scope, Rust calls a special function for us. This function is called drop.

## Move

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);  // error[E0382]: borrow of moved value: `s1`

Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope. 
Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. 
Rust will never automatically create “deep” copies of your data.

## Clone

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // This works, the heap data does get copied.


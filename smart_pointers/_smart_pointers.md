# Smart Pointers
A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data. 
They don’t have any special capabilities other than referring to data. Also, they don’t have any overhead and are the kind of pointer we use most often.

Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities.

Smart pointers are usually implemented using structs. 
The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop traits. 
The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers. 
The Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.
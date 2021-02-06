# Validating References with Lifetimes
every reference in Rust has a lifetime, which is the scope for which that reference is valid. 
Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. 
We must annotate types when multiple types are possible. 
In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. 
Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

## The Borrow Checker
Compille error

    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    } 
    
At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. 
The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.

Fix:

    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }  

x has the lifetime 'b, which in this case is larger than 'a. 
This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.

## Generic Lifetimes in Functions
error[E0106]: missing lifetime specifier

    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute. 
We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes to determine whether the reference we return will always be valid.

The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. 
To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.
    
## Lifetime Annotations in Function Signatures
The longest function definition specifying that all the references in the signature must have the same lifetime 'a

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. 

Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned。

## Lifetime Annotation Syntax

    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime


## Lifetime Annotations in Struct Definitions

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

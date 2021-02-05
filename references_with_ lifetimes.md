# Validating References with Lifetimes
every reference in Rust has a lifetime, which is the scope for which that reference is valid. 
Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. 
We must annotate types when multiple types are possible. 
In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. 
Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

## The Borrow Checker

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

# Packages, Crates, and Modules
## Packages and Crates
A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate. 
A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

create a package

    $ cargo new my-project
     Created binary (application) `my-project` package
    $ ls my-project
    Cargo.toml
    src
    $ ls my-project/src
    main.rs
 
 Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.


If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.


A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.

## Defining Modules
Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).


Create a new library named restaurant by running "cargo new --lib restaurant", src/lib.rs to define some modules and function signatures.

    // define a module by starting with the mod keyword 
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }

Modules can also hold definitions for other items, such as structs, enums, constants, traits, or functions.

## Paths for Referring to an Item in the Module Tree
If we want to call a function, we need to know its path.

A path can take two forms:

* An absolute path starts from a crate root by using a crate name or a literal crate.
* A relative path starts from the current module and uses self, super, or an identifier in the current module.

The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default. Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
you can expose inner parts of child modules' code to outer ancestor modules by using the pub keyword to make an item public.

    mod front_of_house {
        // Moodule must be pub for eat_at_restaurant to call
        pub mod hosting {
            // function must be pub for eat_at_restaurant
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }


## Bringing Paths into Scope with the use Keyword
Bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function

    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        
        //otherwise we have to call: crate::front_of_house::hosting::add_to_waitlist();
    }

You can also bring an item into scope with use and a relative path. 

    use self::front_of_house::hosting;

how to bring two Result types into scope that have the same name but different parent modules and how to refer to them.

    // using the parent modules distinguishes the two Result types.
    use std::fmt;
    use std::io;
    
    fn function1() -> fmt::Result {
        // --snip--
    }
    
    fn function2() -> io::Result<()> {
        // --snip--
    }

### Providing New Names with the as Keyword
we can specify as and a new local name, or alias, for the type

    use std::fmt::Result;
    // won’t conflict with the Result from std::fmt that we’ve also brought into scope. 
    use std::io::Result as IoResult;
    
    fn function1() -> Result {
        // --snip--
    }
    
    fn function2() -> IoResult<()> {
        // --snip--
    }
    
### Re-exporting Names with pub use
When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.

    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }
    
    // By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist. 
    pub use crate::front_of_house::hosting;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
    
### Using Nested Paths to Clean Up Large use Lists
In bigger programs, bringing many items into scope from the same crate or module using nested paths can reduce the number of separate use statements needed by a lot!

    use std::cmp::Ordering;
    use std::io;
    
    // use nested paths to bring the same items into scope in one line. 
    use std::{cmp::Ordering, io};

We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath.

    use std::io;
    use std::io::Write;
    
    // To merge these two paths into one use statement, we can use self in the nested path
    use std::io::{self, Write};

### The Glob Operator
If we want to bring all public items defined in a path into scope, we can specify that path followed by *, the glob operator:

    use std::collections::*;

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

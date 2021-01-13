# Packages, Crates, and Modules
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
 
 

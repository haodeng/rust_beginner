# Customizing Builds with Release Profiles
Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile Cargo uses when you run cargo build --release. 
The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

    $ cargo build
      Finished dev [unoptimized + debuginfo] target(s) in 0.0s
    $ cargo build --release
      Finished release [optimized] target(s) in 0.0s
  
Cargo has default settings for each of the profiles that apply when there aren’t any [profile.*] sections in the project’s Cargo.toml file. 
By adding [profile.*] sections for any profile you want to customize, you can override any subset of the default settings.
  
    // Cargo.toml
    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3

The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. 
Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, 
you’ll want faster compiling even if the resulting code runs slower. That is the reason the default opt-level for dev is 0. 
When you’re ready to release your code, it’s best to spend more time compiling. 
You’ll only compile in release mode once, but you’ll run the compiled program many times, 
so release mode trades longer compile time for code that runs faster. 
That is why the default opt-level for the release profile is 3.

# Publishing a Crate to Crates.io
## Making Useful Documentation Comments
Documentation comments use three slashes, ///, instead of two and support Markdown notation for formatting the text. Place documentation comments just before the item they’re documenting.

    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    
We can generate the HTML documentation from this documentation comment by running cargo doc. This command runs the rustdoc tool distributed with Rust and puts the generated HTML documentation in the target/doc directory.

For convenience, running cargo doc --open will build the HTML for your current crate’s documentation (as well as the documentation for all of your crate’s dependencies) and open the result in a web browser.

## Setting Up a Crates.io Account
you need to create an account on crates.io and get an API token.
    
    cargo login abcdefghijklmnopqrstuvwxyz012345

This command will inform Cargo of your API token and store it locally in ~/.cargo/credentials. Note that this token is a secret: do not share it with anyone else.

## Adding Metadata to a New Crate
Before publishing, you’ll need to add some metadata to your crate by adding it to the [package] section of the crate’s Cargo.toml file. Your crate will need a unique name. 
Filename: Cargo.toml

    [package]
    name = "a unique namee"
    version = "0.1.0"
    authors = ["Your Name <you@example.com>"]
    edition = "2018"
    description = "A fun game where you guess what number the computer has chosen."
    license = "MIT OR Apache-2.0"

## Publishing to Crates.io
Publishing a crate uploads a specific version to crates.io for others to use. Run:

    cargo publish

## Publishing a New Version of an Existing Crate
you change the version value specified in your Cargo.toml file and republish by running "cargo publish"

## Removing Versions from Crates.io with cargo yank
Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. This is useful when a crate version is broken for one reason or another. In such situations, Cargo supports yanking a crate version.

To yank a version of a crate, run cargo yank and specify which version you want to yank:

    $ cargo yank --vers 1.0.1
    
By adding --undo to the command, you can also undo a yank and allow projects to start depending on a version again:

    $ cargo yank --vers 1.0.1 --undo
    
A yank does not delete any code
    
# Cargo Workspaces
## Creating a Workspace
A workspace is a set of packages that share the same Cargo.lock and output directory.
    
    mkdir add
    cd add
    
Next, in the add directory, we create the Cargo.toml file that will configure the entire workspace. 
This file won’t have a [package] section or the metadata we’ve seen in other Cargo.toml files. Instead, it will start with a [workspace] section that will allow us to add members to the workspace by specifying the path to the package with our binary crate;

Filename: Cargo.toml

    [workspace]

    members = [
        "adder",
        "add-one",
    ]

we’ll create the adder binary crate by running cargo new within the add directory. Then generate a new library crate named add-one

    cargo new adder
    cargo new add-one --lib
    
Your add directory should now have these directories and files:

    ├── Cargo.lock
    ├── Cargo.toml
    ├── add-one
    │   ├── Cargo.toml
    │   └── src
    │       └── lib.rs
    ├── adder
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── target
    
In the add-one/src/lib.rs file, let’s add an add_one function:

Filename: add-one/src/lib.rs

    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

Cargo doesn’t assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships between the crates.

Filename: adder/Cargo.toml

    [dependencies]
    add-one = { path = "../add-one" }

Filename: adder/src/main.rs
add a use line at the top to bring the new add-one library crate into scope.

    // bring the new add-one library crate into scope.
    use add_one;

    fn main() {
        let num = 10;
        println!(
            "Hello, world! {} plus one is {}!",
            num,
            add_one::add_one(num)
        );
    }

# Installing Binaries from Crates.io with cargo install
The cargo install command allows you to install and use binary crates locally

This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on crates.io. Note that you can only install packages that have binary targets. A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary, as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs. Usually, crates have information in the README file about whether a crate is a library, has a binary target, or both.

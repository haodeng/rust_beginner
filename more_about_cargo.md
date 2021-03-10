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

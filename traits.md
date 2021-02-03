# Traits: Defining Shared Behavior
We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

## Defining a Trait

    fn main() {
      pub trait Summary {
        // A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.
        fn summarize(&self) -> String;
      }
    }

## Implementing a Trait on a Type

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

Call the methods on instances of NewsArticle and Tweet in the same way we call regular methods:

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

## Default Implementations

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
    // To use a default implementation to summarize instances of NewsArticle 
    // instead of defining a custom implementation, we specify an empty impl block
    impl Summary for NewsArticle {}
    
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // This code prints New article available! (Read more...)
    println!("New article available! {}", article.summarize());

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it. 

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    
    // To use this version of Summary, we only need to define summarize_author when we implement the trait on a type
    // No need to impl default implementations
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    // This code prints 1 new tweet: (Read more from @horse_ebooks...).
    println!("1 new tweet: {}", tweet.summarize());
    
## Traits as Parameters
Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. 
This parameter accepts any type that implements the specified trait. 
In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize. We can call notify and pass in any instance of NewsArticle or Tweet. 

    // the impl Trait syntax, like this:
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

## Trait Bound Syntax

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    
The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases.
For example, we can have two parameters that implement Summary.

    pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    
    pub fn notify<T: Summary>(item1: &T, item2: &T) {
    
## Specifying Multiple Trait Bounds with the + Syntax
We can also specify more than one trait bound. 
we specify in the notify definition that item must implement both Display and Summary. We can do so using the + syntax:

    pub fn notify(item: &(impl Summary + Display)) {
    
    // The + syntax is also valid with trait bounds on generic types:
    pub fn notify<T: Summary + Display>(item: &T) {

## Clearer Trait Bounds with where Clauses
Using too many trait bounds has its downsides. 
functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read.

    // So instead of writing this:
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    
    // we can use a where clause, like this:
    // This function’s signature is less cluttered: 
    // the function name, parameter list, and return type are close together
    fn some_function<T, U>(t: &T, u: &U) -> i32
        where T: Display + Clone,
            U: Clone + Debug
    {

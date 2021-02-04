use std::fmt::Display;
use std::fmt::Formatter;

pub trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing a Trait on a Type
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

impl Display for Tweet {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

// Traits as Parameters
// use the impl Trait syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
// we specify in the notify definition that item must implement both Display and Summary.
pub fn notify_multiple_trait(item: &(impl Summary + Display)) {
    println!("summary: {:?}, to_string: {:?}", item.summarize(), item.to_string());
}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn compare()
{
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // code print: tweet default! (Read more...)
    println!("tweet default! {}", tweet.summarize_default());

    notify(&tweet);
    notify2(&tweet);

    notify_multiple_trait(&tweet);

    let tweet2 = returns_summarizable();
    println!("{}", tweet2.summarize());

    compare();
}
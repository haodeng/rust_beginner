# Traits: Defining Shared Behavior
We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

## Defining a Trait

    fn main() {
      pub trait Summary {
      fn summarize(&self) -> String;
      }
    }

## Implementing a Trait on a Type

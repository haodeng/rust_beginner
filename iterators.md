# Processing a Series of Items with Iterators
In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

    let v1 = vec![1, 2, 3];

    // created an iterator. no iteration takes place here
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

## The Iterator Trait and the next Method
All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this:
    
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
    
The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.

    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // we needed to make v1_iter mutable: calling the next method on an iterator 
        // changes internal state that the iterator uses to keep track of where it is in the sequence.
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    
Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.


## Methods that Consume the Iterator
Methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method

    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
        let v1_iter = v1.iter();

        // the sum method, which takes ownership of the iterator and iterates through 
        // the items by repeatedly calling next, thus consuming the iterator. 
        // As it iterates through, it adds each item to a running total and returns the total when iteration is complete.
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

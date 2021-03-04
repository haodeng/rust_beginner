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

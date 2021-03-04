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

## Methods that Produce Other Iterators
You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

    let v1: Vec<i32> = vec![1, 2, 3];

    // Calling the map method to create a new iterator and 
    // then calling the collect method to consume the new iterator and create a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
    
## Using Closures that Capture Their Environment
a common use of closures that capture their environment by using the filter iterator adaptor.

The filter method on an iterator takes a closure that takes each item from the iterator and returns a Boolean. If the closure returns true, the value will be included in the iterator produced by filter. If the closure returns false, the value won’t be included in the resulting iterator.

    // takes ownership of a vector of shoes and a shoe size as parameters. 
    // It returns a vector containing only shoes of the specified size.
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        
        // call into_iter to create an iterator that takes ownership of the vector. 
        // Then we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true.
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    
    // in_my_size is a vec
    let in_my_size = shoes_in_my_size(shoes, 10);

fn main() {
    loop_iterator();
    iterator_next();
    iterator_sum();
    produce_other_iterator();

    test_filters_by_size();

    calling_next_counter();

    using_other_iterator_trait_methods();
}

fn loop_iterator() {
    let v1 = vec![1, 2, 3];

    // created an iterator. no iteration takes place here
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn iterator_next() {
    let v1 = vec![1, 2, 3];

    // we needed to make v1_iter mutable: calling the next method on an iterator
    // changes internal state that the iterator uses to keep track of where it is in the sequence.
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

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

fn produce_other_iterator() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // Calling the map method to create a new iterator and
    // then calling the collect method to consume the new iterator and create a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// takes ownership of a vector of shoes and a shoe size as parameters.
// It returns a vector containing only shoes of the specified size.
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {

    // call into_iter to create an iterator that takes ownership of the vector.
    // Then we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn test_filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Implementing the Iterator trait on our Counter struct
impl Iterator for Counter {
    // set the associated Item type for our iterator to u32, meaning the iterator will return u32 values.
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Test
fn calling_next_counter() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// take the values produced by an instance of Counter,
// pair them with values produced by another Counter instance after skipping the first value,
// multiply each pair together, keep only those results that are divisible by 3,
// and add all the resulting values together
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
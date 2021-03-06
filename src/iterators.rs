fn main() {
    loop_iterator();
    iterator_next();
    iterator_sum();
    produce_other_iterator();
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
// This code demonstrates different types of iterators in Rust,

fn normal_iterator() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    println!("Using normal iterator:");
    for i in v_iter {
        println!("{}", i);
    }
}

// mutable iterator
// This iterator allows you to modify the elements of the collection
// while iterating over them. You can use this iterator when you need to
// change the values in the collection.
// Note that you need to declare the collection as mutable (using `mut`)
// to use this iterator.
fn mut_iterator() {
    let mut v = vec![1, 2, 3];

    let v_iter = v.iter_mut();

    for i in v_iter {
        *i += 1;
    }
    println!("After mutating iterator:");
    println!("{:?}", v);
}

// next() method
// This method returns the next item from the iterator.
// It returns an Option type, which is Some(value) if there is a next item,
// or None if the iterator is exhausted.
// You can use this method in a loop to manually iterate through the items.
// This is useful when you need more control over the iteration process.
fn dot_next() {
    let nums = vec![1, 2, 3];
    let mut nums_iter = nums.iter();

    println!("Using .next() method:");
    while let Some(num) = nums_iter.next() {
        println!("{}", num);
    }
}

// into_iter() method
// This method takes ownership of the collection and returns an iterator
// that yields owned values.
// After calling into_iter(), the original collection can no longer be used.
// This is useful when you want to consume the collection and work with its values directly.
fn into_iterator() {
    let v = vec![1, 2, 3];

    let v_iter = v.into_iter(); //
                                               // =>  for i in v {}    
    println!("Using into_iter():");            //
    for i in v_iter {                     //
        println!("{}", i);
    }
}

fn main() {
    normal_iterator();
    mut_iterator();
    dot_next();
    into_iterator();
}

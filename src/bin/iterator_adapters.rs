
fn consuming_iterator() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    println!("{:?}", v);
    println!("Using sum() consuming iterator:");
    // sum() consumes the iterator and returns the sum of all its elements.
    let total: i32 = v_iter.sum(); // Note: v_iter can only be used once here

    println!("Total sum: {}", total);
}

fn iterator_adapters() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    let v2_iter = v.iter();

    println!("Using map() iterator adapters:");
    // map() is an iterator adapter that applies a function to each element
    // of the iterator and returns a new iterator with the results.
    let iter1 = v_iter.map(|x| x + 1);

    for x in iter1 {
        println!("{}", x);
    }

    println!("Using filter() iterator adapters:");
    // filter() is another iterator adapter that filters elements based on a predicate.
    // It returns a new iterator that only yields elements for which the predicate returns true.
    let iter2 = v2_iter.filter(|x| *x % 2 == 0);

    for x in iter2 {
        println!("{}", x);
    }

}

fn iter_to_vec() {
    let v = vec![1, 2, 3];

    println!("Using collect() to convert iterator to vector:");
    // collect() is an iterator adapter that collects the elements of an iterator
    // into a collection, such as a vector.
    let iter = v.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);

    let v2: Vec<i32> = iter.collect();

    println!("{:?}", v2);
}

fn main() {
    consuming_iterator();
    iterator_adapters();
    iter_to_vec();
}
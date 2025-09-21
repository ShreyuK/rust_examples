fn main() {
    let max = largest(1, 2);
    let big_char = largest('a', 'b');
    println!("Max int: {}, Max char: {}", max, big_char);
}

// A generic function to find the largest of two values
fn largest<T: PartialOrd>(a: T, b: T) -> T { // T must implement PartialOrd to allow comparison
    if a > b {
        a
    } else {
        b
    }
}

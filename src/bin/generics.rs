fn main() {
    let max = largest(1, 2);
    let big_char = largest('a', 'b');
    println!("Max int: {}, Max char: {}", max, big_char);
}

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

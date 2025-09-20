// Program to demonstrate borrowing ownership in Rust
fn main() {
    let mut s1 = String::from("Shreyas"); // s1 owns the String
    print_string(&mut s1); // Borrow ownership of s1
    // s1 is still valid here
    println!("{} from main", s1);
}

fn print_string(s2: &mut String) {
    // s2 borrows the String
    s2.push_str(" Kotian"); // Modify the borrowed String
    println!("{} from print_string", s2);
    // No need to return ownership, s1 is still valid in main
}


// Rules of Borrowing:
// 1. You can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.
// 3. The scope of a reference must not outlive the data it references.

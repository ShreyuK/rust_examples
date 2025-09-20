// Example of passing ownership in Rust
fn main() {
    let mut s1 = String::from("hello"); // s1 owns the String
    // Move ownership of s1 to print_string and get it back
    s1 = print_string(s1); 
    // Now s1 is valid again
    println!("{} from main", s1);
}

fn print_string(s2: String) -> String { // s2 takes ownership of the String
    println!("{} from print_string", s2); 
    s2  // Return ownership back to the caller
}

// Rules of Ownership:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
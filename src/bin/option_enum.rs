// Code to find the first 'a' in a string
fn find_first_a(s: &str) -> Option<usize> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            Some(index);
        }
    }
    None
}

fn main() {
    let my_string = String::from("Shreyas");

    match find_first_a(&my_string) {
        Some(index) => println!("The first 'a' is at index: {}", index),
        None => println!("No 'a' found in the string."),
    }
}

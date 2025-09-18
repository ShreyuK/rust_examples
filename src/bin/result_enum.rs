// Read a file and manually handle errors using a custom Result enum.
use std::fs::read_to_string;

fn main() {
    let filename = "hello.txt";

    match read_to_string(filename) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
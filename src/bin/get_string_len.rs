fn get_string_len(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    let length = get_string_len(&my_string);
    println!("The length of '{}' is {} characters.", my_string, length);
}
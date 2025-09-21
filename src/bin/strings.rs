fn main() {
    let mut name = String::from("Hello");  // Create a mutable String
    name.push_str(" World");               // Append to the string
    println!("{}", name);
    name.replace_range(5..name.len(), ""); // Remove characters from index 5 to end
    println!("{}", name);

    let s = String::from("Hello World");
    let word = get_first_word(&s);             // word is immutable reference to a slice of s
    println!("{}", word);
}

fn get_first_word(s: &String) -> &str { 
    let mut index = 0;
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    &s[0..index]
}
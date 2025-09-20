use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new(); // Create a new empty HashMap

    users.insert(String::from("Shreyas"), 27); // Insert key-value pairs
    users.insert(String::from("Rohit"), 30);

    let first_user_age = users.get("Shreyas"); // Option<27>

    match first_user_age {
        Some(age) => println!("Shreyas is {age} years old."),
        None => println!("User not found."),
    }
}

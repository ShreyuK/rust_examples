// Demonstrates how to group values by keys using a HashMap in Rust
use std::collections::HashMap;

fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, i32> { 
    // Takes ownership of a vector of (String, i32) tuples
    let mut map = HashMap::new();

    for (key, value) in pairs {
        map.insert(key, value);
    }
    map
}

fn main() {
    let input_vec = vec![
        ("apple".to_string(), 1),
        ("banana".to_string(), 2),
        ("cherry".to_string(), 5),
    ];
    let grouped_map = group_values_by_key(input_vec);

    println!("{:?}", grouped_map);
}

fn main() {
    let mut vec = Vec::new(); // Create a new empty vector
    vec.push(1); // Add elements to the vector
    vec.push(2);
    vec.push(3);
    println!("Original Vector: {:?}", vec);
    even_filter(&mut vec); // Borrow the vector to filter even numbers
    println!("Updated Vector: {:?}", vec); 
}

// Function to filter out odd numbers from the vector
fn even_filter(v: &mut Vec<i32>) {   // Borrow the vector mutably
    let mut i = 0;      
    while i < v.len() { 
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}  
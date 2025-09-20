fn main() {
    let vec = vec![1, 2, 3]; // Create a new vector with initial elements
    println!("Original Vector: {:?}", vec);
    println!("Updated Vector: {:?}", even_filter(vec)); 
}

fn even_filter(v: Vec<i32>) -> Vec<i32> {   // Take ownership of the vector
    let mut new_vec = Vec::new(); // Create a new vector to store even numbers
    for value in v {
       if value % 2 == 0 {
           new_vec.push(value); // Add even numbers to the new vector
       }
    }
    new_vec // Return the new vector
}
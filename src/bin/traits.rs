// A simple trait definition with a default method implementation
// Traits are similar to interfaces in other languages
// They define shared behavior that types can implement

trait Summary {
    fn summarize(&self) -> String{    
        // Default implementation   
        String::from("(Read more...)")
    }
}

// A struct representing a user
struct User {                              
    name: String,
    age: u32,
}

// Implementing the Summary trait for User
impl Summary for User {      
    // Providing a custom implementation for the summarize method             
    fn summarize(&self) -> String {      
        // Overriding the default implementation
        format!("User: {}, Age: {}", self.name, self.age)
    }
}

// A function that takes any type that implements the Summary trait
fn notify(item: &impl Summary) { 
    println!("Notification: {}", item.summarize());
}

fn main() {
    let user = User {
        name: String::from("Shreyas"),
        age: 30,
    };
    println!("{}", user.summarize());
    notify(&user);
}
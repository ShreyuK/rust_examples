// Demonstrates the use of lifetimes in structs
struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}

fn main() {
    let user: User;
    let first_name = String::from("Shreyas");
    {
        let last_name = String::from("Kotian");
        user = User {
            first_name: &first_name,
            last_name: &last_name,
        };
        println!("User's name is: {} {}", user.first_name, user.last_name);        
    } // last_name goes out of scope here
    // This would cause a compile-time error because last_name does not live long enough

    // println!("User's name is: {}", user.first_name);
}

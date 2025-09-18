struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("shreyas123"),
        email: String::from("shreyas@gmail.com"),
        sign_in_count: 1,
    };
    println!("User 1 username: {:?}", user1.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: String::from("example"),
        email: String::from("example@example.com"),
        sign_in_count: 1
    };

    println!("Active: {}", user.active);
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign in count: {}", user.sign_in_count);
}

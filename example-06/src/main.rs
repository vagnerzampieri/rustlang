struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let user = build_user(
        String::from("example"),
        String::from("example@example.com")
    );

    println!("Active: {}", user.active);
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign in count: {}", user.sign_in_count);
}

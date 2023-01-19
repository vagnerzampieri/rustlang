use std::io;

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

    println!("Coloque o username para logar");

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Error reading console");

    println!("Logged? {}", username.trim() == user.username);

    // println!("Active: {}", user.active);
    // println!("Username: {}", user.username);
    // println!("Email: {}", user.email);
    // println!("Sign in count: {}", user.sign_in_count);

    let user2 = User {
        username: String::from("example2"),
        ..user
    };

    println!("Username 2 {}", user2.username);
    println!("Email 2 {}", user2.email);
}

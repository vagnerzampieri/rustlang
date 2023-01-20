#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    DoesNotExist,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
        _ => 0, // () we receive error because the u8 return
    }
}

fn main() {
    println!("Coin: {}", value_in_cents(Coin::Penny));
    println!("Coin: {}", value_in_cents(Coin::Nickel));
    println!("Coin: {}", value_in_cents(Coin::Dime));
    println!("Coin: {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("Coin: {}", value_in_cents(Coin::DoesNotExist));
}

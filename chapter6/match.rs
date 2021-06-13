#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime, 
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // match works line a switch in other programming languages
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { //inside the match is where we can extract values
            println!("State: {:?}",state);
            25
        },
        // _ => (), we can use _ to match all the remaining variants instead of typing them all
    }
}

fn main() {
    value_in_cents(Coin::Quarter(State::Alabama));
}
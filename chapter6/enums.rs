enum Ip {
    V4(u8,u8,u8,u8), //enums can contain information in order to work like structs
    V6(String),
}

fn route(ip: Ip) {} //this function can be called with either enum type

enum Message { 
    Quit,
    Move {x: i32, y:i32}, //this enum type works like a struct
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message { //enums can also have implementations
    fn call(&self) {
        // do something with the information
    }
}

fn main() {
    let home = Ip::V4(127,0,0,1);
    let loopback = Ip::V6(String::from("::1"));

    let msg = Message::Write(String::from("Hello"));
    msg.call();
}
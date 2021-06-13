struct User { //Declare a struct
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Point(i32,i32,i32); //tuple struct

fn build_user(username: String, email: String) -> User {
    User {
        email, //if the parameter and the field have the same name
        username, //we only need to write it once
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User { //instantiate a struct
        email: String::from("email@mail.com"),
        username: String::from("user"),
        active: true,
        sign_in_count: 1,
    };

    // struct elements can be accessed using dot notation
    user1.email = String::from("email2@mail.com");

    let user2 = User {
        email: String::from("email3@mail.com"),
        username: String::from("user2"),
        ..user1 //set the remaining fields to be the same as user1
    };

    let origin = Point(0,0,0); //basically a named tuple
}
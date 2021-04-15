fn main() {
    let s1 = gives_ownership(); //moves its return value to s1

    let s2 = String::from("s2"); //s2 comes into scope

    let s3 = takes_and_gives_back(s2);  //s2 is moves to the function and returned to main
                                        //the value of s2 can be accessed in s3 only

    println!("s1: {}, s3: {}",s1,s3);
}

fn gives_ownership() -> String { //moves its return value to the function that calls it
    let st = String::from("Hello");
    st
}

fn takes_and_gives_back(st: String) -> String { //takes ownership of a string and gives it back
    st
}
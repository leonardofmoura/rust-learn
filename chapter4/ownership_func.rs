fn main() {
    let s = String::from("Hello");

    takes_ownership(s); //s is moved into the function

    //s is no longer valid here because the end of the 
    //previews function called drop on it 

    let x = 5;

    makes_copy(x); //x goes into the function
    
    //i32 is a Copy type, so no drop is called after the function exits
    //making it ok to use it here

    println!("Still x: {}",x);
}

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("Str: {}",some_string);
} //some_string comes out of scope and its 'drop' is called

fn makes_copy(some_integer: i32) { //some_integer comes into scope
    println!("int: {}",some_integer);
} //some_integer comes out of scope
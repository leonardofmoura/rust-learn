fn main() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1); //'&' indicates we are passing a reference

    println!("The len of {} is {}",s1,len); //we can use s1 after the function call

    //Mutable references
    let mut s2 = String::from("Hello 2"); //Create mutable variable

    change(&mut s2); //pass a mutable reference (we can not have more than one mutable reference at a time)
    //we cant also have a mutable reference after we declare a unmutable reference because this reference
    //is not expected to be changed in its lifetime

    //references must also be always valid, so there are no dangling references

    println!("s2: {}",s2); // it is changed here
}


fn calculate_len(s: &String) -> usize {     //s is a reference to a string (references are immutable)
    s.len()                                //because of the reference, this function 
}                                          //never owns s, so it is not dropped

fn change(s: &mut String) { //In this function we accept a mutable reference
    s.push_str("world!");
} 
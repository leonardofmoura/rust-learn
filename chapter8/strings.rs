#![allow(unused)]
fn main() {
    let mut s1 = String::new(); // create empty String

    // We can create String objects from str literals
    let data = " Content";
    let s2 = data.to_string();
    let s3 = "directly".to_string();
    let s4 = String::from(data);
    
    s1.push_str("Hell"); // push a string literal
    s1.push('o'); // push a character

    // we can concat two strings with '+', but the operator
    // takes ownership of the first string, rendering it unusable
    let s5 = s1 + &data;

    // format! is another way to concat strings
    let s6 = format!("{}{}",s3,data);

    // we can iterate strings using the .chars method
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // we can iterate strings using the .bytes method
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    
    println!("s2: {}",s2);
    println!("s3: {}",s3);
    println!("s4: {}",s4);
    println!("s5: {}",s5);
    println!("s6: {}",s6);
}
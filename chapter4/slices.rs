fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i,&item) in bytes.iter().enumerate(); {
        if item == b' ' {
            return &s[..i]; //this is a string slice, returns a reference from 
        }                   // the start of the string to i
    }

    &s[..] //Slice of the whole string
}

fn main() {
    let s = String::from("Hello World");

    let wd = first_word(s);

    println!("Wd: {}",wd);
}
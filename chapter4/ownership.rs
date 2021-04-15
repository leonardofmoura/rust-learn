fn main() {
    let s1 = String::from("Hello"); //Allocated in the heap
    let s2 = s1; //s1 is moved to s2 -> s1 can not be used anymore

    let s3 = s2.clone(); //make a hard copy of s2

    println!("s1: {}, s3: {}", s2, s3);

    let x = 5; //Types that are stored on the stack can be 
    let y = x; // used after they are stored in another variable

    println!("x: {}, y: {}",x,y);
} //all active variables go out of scope and are dropped
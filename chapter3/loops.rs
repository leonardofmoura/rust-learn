fn main() {
    let mut counter = 0;

    //loop keyword
    let result = loop { //infinite loop
        counter += 1;

        if counter % 10 == 0 {
            break counter * 2 //the loop will return 20
        }
    };
    println!("Result: {}\n",result);

    //while loop
    let mut number = 3;
    while number != 0 {
        println!("number: {}",number);
        number -= 1;
    }
    println!("Liftoff\n");

    //for and looping through collections
    let arr = [1,2,3,4,5,6];
    for element in arr.iter() {
        println!("Element: {}",element);
    }
    println!();

    //ranges in for loops
    for num in (1..5).rev() { //rev reverses the order
        println!("num: {}",num)
    }
}

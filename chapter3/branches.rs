fn main() {
    let number = 7;

    if number < 5 { //Conditions must be bools
        println!("Condition True");
    }
    else {
        println!("Condition false");
    }

    //else ifs
    let y = 3;
    if y % 4 == 0 {
        println!("y is divisible by 4");
    }
    else if y % 3 == 0 {
        println!("y is divisible by 3");
    }
    else if y % 2 == 0 {
        println!("y is divisible by 2");
    }
    else {
        println!("y is not divisible by 4,3 or 2");
    }

    //we can use an if on the right side of a let
    let condition = true;
    let z = if condition {5} else {2}; //Values must have the same type
    println!("z: {}",z);
}

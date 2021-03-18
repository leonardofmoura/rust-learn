fn main() {
    another_function(5,7);

    //==== EXPRESSIONS AND STATEMENTS =======
    let _x = 5; //Statement -> instruction
    let _y = {  //Braces: Expression -> can be evaluated
        let x = 3;
        x + 1  //No ; -> Means the result is returned
    };

    println!("P1: {}",plus_one(2));
}

//====== FUNCTION DECLARATION ===========
fn another_function(x: i32, y: u8) {
    println!("x: {}, y: {}",x,y);
}

//====== FUNCTION RETURN VALUES =========
fn plus_one(x: i32) -> i32 {
    x + 1   //No return is needed, rust implies the last value with no ;
            //in the function to be the return
}

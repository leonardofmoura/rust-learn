fn main() {
    //===== VARIABLES AND TYPES ===========
    //Variables are immutable by default
    //They need to be declared with 'mut' to be mutable
    let mut x = 5;
    println!("The value of x is {}",x);
    x = 6;
    println!("The value of x is {}",x);

    //Shadowing - using let with a variable that exists, it 
    //creates a variable with the same name
    //Types cane be annotated -> Sometimes they must be because the 
    //compiler needs them

    let y: i32 = 5;
    let y = y + 1;
    let y = y * 2;
    println!("y: {}",y);

    //We can change types when shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces: {}",spaces);


    //===== TUPLES ===========
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Tuples can be deconstructed
    let (x, y ,z) = tup;
    println!("x: {}, y:{}, z:{}",x,y,z);

    //Tuple member access
    let tupx = tup.0;
    println!("Tupx: {}",tupx);

    
    //===== ARRAYS ===========
    let a: [i32;5] = [1,2,3,4,5];
    let fst = a[0];
    let scnd = a[1];
    println!("fst: {}, scnd: {}",fst,scnd);
}   


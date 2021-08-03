fn main() {
    let mut v: Vec<i32> = Vec::new(); // we need a type annotation to create an empty vector
    let mut v2 = vec![1,2,3,4,5];   // we can create vectors with values inside with the vec! macro. 
                                // it infers the type of the vector

    v.push(5); // after we push information into the vector, rust can infer the type of the data in the vector

    let third: &i32 = &v2[2];
    println!("Third: {}",third);

    match v2.get(2) {
        Some(t) => println!("Third: {}",t),
        None => println!("No third element"),
    }

    // we use 'for' to iterate vectors
    for i in &v2 {
        println!("{}",i);
    }

    // using mutable references we can modify the values inside the vector
    for i in &mut v2 {
        *i += 50;
    }

    // using enums we can store different types inside a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }


    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Blue"))
    ];

    for i in &row {
        println!("{:?}",i);
    }
} // all vectors and their elements come out of scope here
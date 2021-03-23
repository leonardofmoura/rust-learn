//Classic simple program to generate a number in the 
//Fibonacci series

use std::io;

fn main() {
    println!("====== FIBONACCI SEQUENCE ======");
    println!("Write the order of the series");

    let mut inpt = String::new();
    io::stdin().read_line(&mut inpt).expect("Invalid Input");

    let num: u32 = match inpt.trim().parse() {
        Ok(n) => n,
        Err(_) => 0 //Invalid input simply returns 0
    };

    println!("\nFib[{}] = {}",num,fib(num));
}

fn fib(num: u32) -> u32 {
    let mut f0 = 0;
    let mut f1 = 1;
    let mut fib = 1;

    for n in 1..num+1 {
        fib = f0 + f1;
        f0 = f1;
        f1 = fib;

        println!("Fib[{}]: {}",n,fib);
    }

    fib
}
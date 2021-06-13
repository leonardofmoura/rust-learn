
fn print_num(num: Option<u8>) {
    if let Some(n) = num {
        println!("{}",n);
    }
    else { // else is used to match everything that's not matched in the if
        println!("None");
    }
}


fn main() {
    let n1 = Some(3u8);
    let n2 = None;

    print_num(n1);
    print_num(n2);
}
// Rust does not have Nulls so it uses the Option enum
// in order to represent this behavior
// It has the definition:
// enum Option<T> {
//     Some(T),
//     None,
// }
// and it's brought automatically into scope

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let some_number = Some(5);
    let some_str = Some("a string");

    let none: Option<i32> = None;

    let six = plus_one(some_number);
    let none = plus_one(None);
}
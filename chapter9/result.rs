use std::{fs::File, io::ErrorKind};


fn main() {
    let filename = "hello.txt";

    let f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}",error);
            })
        }
        else {
            panic!("Problem opening the file: {:?}",error);
        }
    });
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// a struct can have multiple impl blocks
impl Rectangle { // The impl block allows us to define methods associated to the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { //if self is not specified this is an association:
        Rectangle {                     //a function associated with the struct itself and 
            width: size,                //not an instance. It is called with Rectangle::square
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 10,
        height: 40, 
    };

    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {}",
        rect.area()
    );

    println!("can rect hold r2? {}", rect.can_hold(&r2));

    println!("Sq: {:?}",sq);
}

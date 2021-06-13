// Without struct or tuples

fn area_1(width: u32, height: u32) -> u32 {
    width * height
}


fn main_1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "1 - The area of the rectangle is {} square pixels",
        area_1(width1,height1)
    );
}

//=========== Tuples ==========================

fn area_2(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main_2() {
    let rect1 = (30,50);

    println!(
        "2 - The area of the rectangle is {} square pixels",
        area_2(rect1)
    );
}

// ======== Structs =============================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main_3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "3 - The area of the rectangle is {}",
        area_3(&rect1)
    );

    println!(
        "rect1 is {:?}",
        rect1
    );
}

// ========= Main ===============================

fn main() {
    main_1();
    main_2();
    main_3();
}
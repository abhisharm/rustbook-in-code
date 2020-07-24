#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    // writing a program that calculates the area of a rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area((width1, height1))
        area(&rect1)
    );

    // would nice to be able to print the rectangle without formatting
    // '{:?}' prints debug output instead
    // println!("rect1 is {:?}", rect1);
    // '{:#?}' prints in a 'prittified' way
    println!("rect1 is {:#?}", rect1);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// a version of the function that uses tuples:
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// we use an immutable borrow of the struct rectangle rather than
// taking ownership of it - this enables us to continue using the
// rectangle even after the 
fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

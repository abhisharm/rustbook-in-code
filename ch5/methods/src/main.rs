#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    // note we add the '&' before self, because methods can take ownership of self in rust
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // an 'associated function' - sort of like a static method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// a struct can have multiple 'impl' blocks:
impl Rectangle {
    fn distance(&self, other: &Rectangle) -> f64 {
        let mut dh2: u32 = 0;
        let mut dw2: u32 = 0;

        if self.height > other.height {
            dh2 = (self.height - other.height).pow(2);
        } else {
            dh2 = (other.height - self.height).pow(2);
        }

        if self.width > other.width {
            dw2 = (self.width - other.width).pow(2);
        } else {
            dw2 = (other.width - self.width).pow(2);
        }
        
        ((dh2 + dw2) as f64).sqrt()
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 50, 
        height: 60,
    };

    let rect3 = Rectangle {
        width: 45,
        height: 55,
    };

    println!(
        "Can rect2: {:?} fit inside rect3: {:?}?.\n The answer is: {}",
        rect2, rect3, rect3.can_hold(&rect2)
    );

    println!(
        "the 'distance' between rect2 and rect3 is: {}/{}",
        rect2.distance(&rect3),
        rect3.distance(&rect2));

    let square = Rectangle::square(10);

    println!(
        "The area of a square of size {} is {}",
        10, square.area());

    let rect0 = Rectangle {
        height: 0, 
        width: 0,
    };

    let rect54 = Rectangle {
        height: 3,
        width: 4,
    };

    println!("The hypotenuse of a right triangle with sides 3 and 4 is: {}",
        rect0.distance(&rect54));
}



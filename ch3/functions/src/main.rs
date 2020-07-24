fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    expressions();
    return_value();
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expressions() {
    let x = 5;
    
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// functions with return values
fn return_value() {
    let x = five();
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

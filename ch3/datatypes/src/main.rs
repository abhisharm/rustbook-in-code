fn main() {
    // the type-annotation here is required
    let guess: u32 = "42".parse().expect("Not a number!");

    // number literals:
    let decimal = 98_222; // '_' can be used as a visual separator
    let hex = 0xff;
    let oct = 0o77;
    let b = 0b1111_0000;
    let byte = b'A';

    // we can also use suffixes for any integer literal except byte:
    let a = 78u32;
    let b = -100i16;
    let c = 1u128;
    let d = 100_000i128;

    // wrapping behavior causes a panic in debug mode:
    // compile with '--release' to allow it to run
    let wrap = 255u8;
    let wrap = wrap + 2u8;
    println!("The wrapped value is: {}", wrap);

    // floating-points
    let x = 2.0; // f64 - default

    let y: f32 = 3.0; // f32

    // booleans:
    let t = true;

    let f: bool = false; // explicitly annotated

    // character types
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
    // rusts unicode is 4 bytes, meaning it can represent alot of chrs

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // pattern-matching to get the values out
    
    println!("The value of y is: {}", y);

    // we can also access a tuple by using a '.'
    let five_hundred = tup.0;
    let one = tup.2;
    println!("{} {}", five_hundred, one);

    // arrays!
    let a = [1, 2, 3, 4, 5]; // arrays have fixed-length
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // the number 3 repeated 5 times
    let first = a[0];
    let second = a[1];
    println!("First is {}, second is {}", first, second);

    // out of bounds accesses will compile, but throw runtime errors
    let index = 10;
    let element = a[index];
    
    println!("The value of element is: {}", element);
}


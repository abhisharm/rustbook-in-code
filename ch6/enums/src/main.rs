#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Loopback: {:?}\nhome: {:?}", loopback, home);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}\nLoopback: {:?}", home, loopback);

    let m = Message::Write(String::from("This is a test message"));
    m.call();

    // using Option
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("some_number {:?}, some_string: {:?}, none i32: {:?}", some_number, some_string, absent_number);

    // The benefit of using Option<T> is that the compiler treats Option<T> and T differently
    // So we can't have a null reference that looks the same as a non-null reference to 
    // the user
    let x: i8 = 5;
    let y = Some(5i8);

    // This causes a compilation error, since i8 and Some(i8) are different types!
    // let sum = x + y;
}

// we can also mix values within enums
// we can put any type inside of an enum, even another enum!
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// mixing all kinds of enum definitions
enum Message {
    Quit,
    Move { x : i32, y : i32 }, // no parenthesis?
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can also define methods on enums
impl Message {
    fn call(&self) {
        println!("Message was called");
    }
}

// Option enum
// enum Option<T> {
// Some(T),
// None,
// }
// included in the prelude

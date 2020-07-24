fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);
    println!("new string: {}", s);

    let mut s = String::from("Abhishek");

    // a single mutable variable can only have one mutable reference
    // in-scope

    let r1 = &mut s;
    let r2 = &mut s;

    // println!("{}, {}", r1, r2); fails
    
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; // this is fine - different scope
    
    // the same rules apply with mixing mutable & immutable 
    // references
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // as long as the mutable references are used before the 
    // immutable reference, you have have multiple:
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // the rust compiler ensures we don't have dangling references,
    // i.e., references to some data that is out of scope
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // this function enables us to reference the object without taking
    // ownership of it
    s.len()
}// here, 's' goes out of scope, but because it's a pointer and not
// the object itself, it doesn't prevent us from using 's' later

fn change(some_string: &mut String) {
    // this function call doesn't work, as it is not allowed to 
    // change a value that you are borrowing
    some_string.push_str(", world");
    // basically, you can't change ownership of a borrowed value,
    // unless the borrowed value (reference) is a mutable one
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

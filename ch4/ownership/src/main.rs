fn main() {
    // Rules about ownership:
    // 1. Each value in rust has a variable that is its 'owner'
    // 2. A value can only have one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped
    
    {
        let _s = "hello"; // the variable 's' is in-scope
    } // after this, 's' is out of scope

    // sometimes, we need a more complex data-type to represent a string
    // in this case, we use the 'String' type
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a string
    println!("{}", s); // prints 'hello, world!'

    variables();
    functions();
    return_values();
    workaround();
}

fn variables() {
    let x = 5;
    let y = x;

    // we would think that this performs a simple copy as above,
    // but this is not the case. Since the string itself exists on
    // the heap, the 'String' data type only contains a pointer to
    // the string data. This means that both variables reference the
    // same data
    let s1 = String::from("hello");
    let s2 = s1;

    // this should cause an error, since s1 is out of scope
    // println!("{}, world!", s1)

    // the concept of ownership implies that all copying is shallow
    // the below code works correctly because it performs a deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // the below code works because primitives live on the stack
    println!("x = {}, y = {}", x, y);
}

fn functions() {
    // the semantics for passing a value to a function are similar
    // to those for passing a value to a variable. 
    
    let s = String::from("hello"); // s comes into scope
    println!("s: {}", s); // this is fine bc ownership hasn't changed
    takes_ownership(s); // the value for s has been passed to the fn
    // println!("s: {}", s); // this doesn't work because out of scope

    let x = 5; // x comes into scope
    makes_copy(x); // x has been copied & is still usable
    println!("x: {}", x); // fine because x is still in scope
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn return_values() {
    let s1 = gives_ownership(); // gives ownership moves its value
                                // into s1
    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // takes and gives back moves
                                       // ownership of s2 to the 
                                       // function, then gives owner
                                       // ship to 's3'
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn workaround() {
    // we return a value every time its passed to a function as part
    // of a tuple as a workaround for changing ownership
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("the length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns length of the string

    (s, length)
}

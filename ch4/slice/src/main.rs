fn main() {
    // small programming problem:
    // write a function that takes a string and returns the
    // first word it finds in the string. If the function doesn't
    // find a space in the string, the entire string should be
    // returned

    // string slices:
    let s = String::from("hello world!");
    
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s);

    // using slices prevents us from getting a reference to an 
    // object that is out of scope
    // s.clear(); // error! - this attempts to get a mutable reference
               // even though there is aready an immutable one

    println!("the first word is: {}", word);
    
    // other kinds of slices:
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // has type &[i32]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

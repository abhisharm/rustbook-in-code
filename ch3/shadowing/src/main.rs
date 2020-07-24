fn main() {
    // shadowing occurs when we use the 'let' keyword to reassign a 
    // variable to something else
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is {}", x);

    // Shadowing is different from reassigning a 'mut' variable as
    // it essentially creates a new variable, and requires us to use
    // the 'let' keyword.
    //
    // Also, shadowing lets you change the type of a variable
    
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces has {} charactes", spaces);
}

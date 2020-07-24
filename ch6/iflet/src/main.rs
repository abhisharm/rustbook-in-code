fn main() {
    // the 'if let' syntex gives us less verbose, consise control-flow
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // we only want to do something where the Some(u8) is Some(3)
    // an easier way to do this is using if let:
    if let Some(3) = some_u8_value {
        println!("three!");
    }

    // we can also add an 'else' statement to 'if let'
    // essentially the same as adding '_ => ...'
    let mut count = 0;
    for i in (0..11) {
        if let 10 = count {
            println!("Reached 10!");
        } else {
            println!("counting...");
            count += 1;
        }
    }
}

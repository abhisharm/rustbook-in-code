#[derive(Debug)] // so we can inspect the state later
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    // --snip -- 
    Wyoming,
}

// From 1999 to 2008 the US mint minted quarters with different designs for each state on a side
// So we change our enum to have the quarter include a UsState value stored in it
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // this is a match expression - used as the return value for the function
    // note that there is no ';' at the end of the expression
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// matching on Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Arkansas));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("New value: {}", match six { Some(i) => i, _ => 0});

    // The '_' wildcard => lets us match all other possible inputs exhaustively
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one!"), 
        3 => println!("three!"),
        5 => println!("five!"),
        7 => println!("seven!"),
        _ => (),
    }
}

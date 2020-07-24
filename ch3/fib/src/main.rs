fn main() {
    let third = fib(3);

    let twenty_two = fib(22);

    // let hundred = fib(100); - overflow

    println!("The third fibonacci number is: {}", third);
    println!("The twenty-second fibonacci number is: {}", twenty_two);
    // println!("The hudreth fibonacci number is: {}", hundred);
}

fn fib(n: u32) -> u32 {
    let mut last: u32 = 0;
    let mut cur: u32 = 0;
    for i in (0..n) {
        if i == 0 {
            cur = 0;
        } else if i == 1 {
            cur = 1;
        } else {
            let tmp: u32 = last + cur;
            last = cur;
            cur = tmp;
        }
    }
    cur
}

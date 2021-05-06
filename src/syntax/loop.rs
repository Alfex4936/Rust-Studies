#![warn(while_true)]
fn main() {
    // while true with label
    'outer: loop {
        loop {
            println!("Hello!");
            break 'outer;
        }
    }

    // `loop` can return a value to `break`, `while` and `for` can't
    let mut total_sum = 0;
    let result1 = 'main: loop {
        for i in 0..10000 {
            total_sum += i;
            if i == 100 {
                break 'main total_sum * 2; // break main and result1 will be total_sum * 2
            }
        }
    };

    println!("Result1 is {}", result1);

    while true {
        println!("just while true");
        break;
    }
    let mut result = false;
    while !result {
        result = true;
        println!("True or False");
    }
}

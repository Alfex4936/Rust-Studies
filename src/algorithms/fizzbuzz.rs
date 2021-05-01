use std::io;
use std::io::Write;

fn main() {
    print!("Enter n: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            // println!("{} bytes read", n);
            let n = input.trim().parse::<u32>().unwrap();
            println!(">>> Method 1");
            fizzbuzz_one(n);
            println!(">>> Method 2");
            fizzbuzz_two(n);
            println!(">>> Method 3");
            fizzbuzz_three(n);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn fizzbuzz_one(n: u32) {
    for i in 1..n {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn fizzbuzz_two(n: u32) {
    let mut status: bool = false;
    for i in 1..n {
        status = false;
        if i % 3 == 0 {
            println!("Fizz");
            status = true;
        }
        if i % 5 == 0 {
            println!("Buzz");
            status = true;
        }
        if !status {
            println!("{}", i);
        }
    }
}

fn fizzbuzz_three(n: u32) {
    let mut three = 0;
    let mut five = 0;
    let mut status: bool = false;

    for i in 1..n {
        status = false;
        three += 1;
        five += 1;

        if three == 3 {
            println!("Fizz");
            status = true;
            three = 0;
        }
        if five == 5 {
            println!("Buzz");
            status = true;
            five = 0;
        }

        if !status {
            println!("{}", i);
        }
    }
}

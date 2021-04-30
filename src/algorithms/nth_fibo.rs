use std::io;
use std::io::Write;

macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

fn main() {
    print!("Enter n: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            // println!("{} bytes read", n);
            let fibo: u32 = calculate(input.trim().parse::<u32>().unwrap());
            println!("N-th fibonacci: {}", fibo);
        }
        Err(error) => println!("Error: {}", error),
    }
}

fn calculate(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    let mut counter = 3;
    while counter <= n {
        let next = first + second;
        first = second;
        second = next;
        counter += 1;
    }
    ternary!(n > 1 => second; first)
}

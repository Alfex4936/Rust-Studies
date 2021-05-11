use std::io;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("{} bytes read", n),
        Err(error) => println!("error: {}", error),
    }

    input = String::new();
    // read_line returns Result<usize>
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("{} bytes read", n),

        _ => println!("Error"), // catch all patterns that are matched
    }

    match "abcd" {
        "a" | "b" | "c" => println!("a or b or c"), // many cases
        "abcde" => println!("abcde"),
        _ => println!("Error"),
    }
}

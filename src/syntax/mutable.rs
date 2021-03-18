use std::io;
use std::io::Write;

// read_line은 &mut String 받음
fn main() {
    print!("Enter your weight (kg): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            let mars_weight: f32 = calculate(input.trim().parse::<f32>().unwrap());
            println!("Weight on Mars: {}kg", mars_weight);
        }
        Err(error) => println!("error: {}", error),
    }
}

fn calculate(weight: f32) -> f32 {
    (weight / 9.81) * 3.711 // return keyword 도 있음
}

mod folder;
mod hello; // Rust treats it as a file name, so it'll look up hello.rs

use folder::Car;
use hello::Person;

fn main() {
    let james = Person::new("james".to_string());
    println!("Hello {}!", james.get_name());

    let benz = Car::new(7000);
    println!("Price is {}", benz.get_price());
}

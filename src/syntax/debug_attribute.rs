use std::fmt;

fn main() {
    let a = Home::new("Rust".to_string());
    dbg!(a);
}

// #[derive(Debug)]
struct Home {
    name: String,
}

impl Home {
    fn new(name: String) -> Home {
        Home { name: name }
    }
}

impl fmt::Debug for Home {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Home name is {}", self.name)
    }
}

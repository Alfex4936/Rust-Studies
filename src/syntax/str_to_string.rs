struct Human {
    name: String,
}

impl Human {
    fn new<I: Into<String>>(name: I) -> Self {
        Human { name: name.into() }
    }
}

fn main() {
    let human = Human::new("string name");

    println!("Human name: {}", human.name)
}

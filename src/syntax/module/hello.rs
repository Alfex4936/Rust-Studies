pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Self {
        Person { name: name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

struct Student {
    org: String,
    name: String,
    age: i32
}

// struct 메소드 구현
impl Student {
    fn get_age(&mut self) {  // &self나 borrow해서 &mut
        self.age += 1;
    }

    fn find_age(&self) -> i32 {
        100 - self.age
    }
}

fn main() {
    let mut student1 = Student{org: String::from("ABC"), name: String::from("Lemon"), age: 99};
    let student2 = Student{org: String::from("ABC"), name: String::from("Dalcom"), age: 25};

    println!("{} goes to {} university and age is {} years old", student1.org, student1.name, student1.age);
    
    student1.get_age();

    println!("{} is now {} years old", student1.name, student1.age);
    println!("{} will have {} years to be 100 years old", student2.name, student2.find_age());
}

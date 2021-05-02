use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

impl<T> DerefExample<T> {
    fn new(value: T) -> DerefExample<T> {
        DerefExample { value }
    }
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Drop for DerefExample<T> {
    fn drop(&mut self) {
        println!("About to go out of scope.");
    }
}

fn main() {
    let a = Box::new(String::from("hello"));
    let b = &a;
    let x = DerefExample { value: 33 };
    let y = DerefExample::new('a');

    println!("{}", &(*a)[0..4]); // hell
    println!("{}", *b); // hello
    drop(x); // ! ERROR: value moved to here
    println!("{}", *x); // 33
    println!("{}", y); // ! ERROR: `DerefExample<char>` cannot be formatted with the default formatter

    // About to go out of scope. will be printed twice as of x and y
}

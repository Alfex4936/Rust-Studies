fn main() {
    let a = String::from("Hello");
    println!("{}", a.print());

    let buf: &[u8] = &[0u8; 4];
    println!("{}", buf.print());
}

trait Print {
    fn print(&self) -> String;
}

impl Print for String {
    // add functionalities to String type
    fn print(&self) -> String {
        "I'm a string".to_string()
    }
}

impl Print for &[u8] {
    // add functionalities to slice
    fn print(&self) -> String {
        "I'm a slice".to_string()
    }
}

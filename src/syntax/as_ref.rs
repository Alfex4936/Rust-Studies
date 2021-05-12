// Used to do a cheap reference-to-reference conversion.
#![warn(unused_variables)]
struct MyType(String);

impl AsRef<str> for MyType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

fn greet(msg: &str) {
    println!("Hello, {}!", msg);
}

fn main() {
    let x = MyType(String::from("example"));
    greet(x.as_ref());

    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {:?}", text.unwrap());
}

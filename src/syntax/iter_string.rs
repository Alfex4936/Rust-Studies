fn main() {
    let string = "hello world";

    // Way 1
    let mut iter = string.chars();
    loop {
        let item = iter.next();
        match item {
            Some(char) => {}
            None => break,
        }
    }

    // Way 2

    for (i, c) in string.chars().enumerate() {
        if c == ' ' {
            // blah
        }
    }
}

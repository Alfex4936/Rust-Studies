use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let wordlist_raw = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let split = wordlist_raw.split("\n");
    let wordlist_vec: Vec<&str> = split.collect();

    // print all words
    for word in wordlist_vec.iter() {
        println!("Word: {}", word);
    }
}

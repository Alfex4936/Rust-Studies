use std::io;

// 1. 각 값들은 변수에 의해 owned
// 2. owner가 scope에서 나가면, 값은 자동으로 deallocate
// 3. ONE owner at a time.
fn main() {
    let mut input = String::new();
    // let mut s = input;  // ! error

    // double free error 방지됨, input은 더 이상 사용할 수 없어짐
    // copy가 가능한 type, borrow가 가능한 type이 다름
    let a = 5;
    let b = a; // Stack

    //std::string::String, does not implement the `Copy` trait
    fnc(input); // ! error
                // 이후에 또 값을 쓰면, borrowing없이는 이 변수는 dead memory
    io::stdin().read_line(&mut input); // ! error
}

fn fnc(s: String) {}

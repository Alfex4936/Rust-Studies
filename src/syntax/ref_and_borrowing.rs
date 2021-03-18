use std::io;

fn main() {
    let mut input = String::new();
    let mut one = &mut input; // Reference
    let two = &input; // ! error - mutable/immutable 상태 둘다로 borrow 불가능

    // ----------------------------------------------------------------
    let one = &input
    let two = &input
    fnc(input);  // 컴파일러는 one,two가 사용 안될 걸 앎

} // data races 컴파일 타임에 방지

fn fnc(s: String) {}

fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let b: [i32; 5] = [5, 4, 3, 2, 1]; // 타입 미리 지정
    let c = [3; 5]; // 3을 5개 만들어라

    println!("{:?}", a); // [1, 2, 3, 4, 5]
    println!("{:?}", b); // [5, 4, 3, 2, 1]
    println!("{:?}", c); // [3, 3, 3, 3, 3]

    give_me_fixed_arr([1, 2, 3, 4]); // error [E0308]
    give_me_any_arr(&[1, 2, 3, 4]);
}

fn give_me_fixed_arr(a: [u8; 5]) {} // only take an array that has 5 elements
fn give_me_any_arr(a: &[u8]) {} // doesn't have to care about size

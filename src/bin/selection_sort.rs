extern crate fastrand;

use std::iter::repeat_with;
use std::time::Instant;

fn main() {
    println!("Hello, Selection Sort!");

    let mut v: Vec<i32> = repeat_with(|| fastrand::i32(..)).take(10000).collect();

    let start = Instant::now();
    selection_sort(&mut v);
    println!("Elapsed time: {:.2?}", start.elapsed());
    // println!("{:?}", v);
}

fn selection_sort(array: &mut Vec<i32>) {
    let n = array.len();
    for i in 0..n - 1 {
        let mut smallest = i;
        for j in i + 1..n {
            if array[j] < array[smallest] {
                smallest = j;
            }
        }
        array.swap(i, smallest);
    }
}

extern crate rand;

mod utils;

fn main() {
    println!("Hello, Insertion Sort!");
    let mut test = make_array(30);
    insert_sort(&mut test);
}


fn insert_sort(array: &mut [i32]) {
    let length = array.len() - 1;
    for i in 1..length {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1); 
            j -= 1;
        }
    }
}
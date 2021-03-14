macro_rules! test {
    ($msg:expr) => {
        println!("{}", $msg)
    };
}

macro_rules! print_array {
    ($msg:expr) => {
        println!("{:?}", $msg);
    }
}

fn main() {
    println!("Hello, world!");

    test!("hi");

    let mut test = [-11, 1, 2, -14, 4, 5, 6, 0];
    bubble_sort(&mut test);

    print_array!(test);
}


fn bubble_sort(array: &mut [i32]) {
    let mut is_sorted = false;
    let mut count = 0;
    let length = array.len() - 1;
    while !is_sorted {
        is_sorted = true;
        for i in 0..length - count {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                is_sorted = false;
            }
        }
        count += 1;
        
    }
}
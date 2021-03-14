use rand::Rng;

#[macro_export]
macro_rules! print_array {
    ($msg:expr) => {
        println!("{:?}", $msg);
    }
}

pub fn make_array(size: i32) -> Vec<i32>{
    let arr: Vec<i32> = (0..100).map(|_| rng.gen_range(0, 20)).collect();
    arr
}
        

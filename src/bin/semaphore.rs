use lazy_static::lazy_static;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref ARRAY: Mutex<Vec<u32>> = Mutex::new(vec![1; 5]);
}

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}

fn lifecycle_of_philosohper(id: u32) {
    loop {
        contemplate();
        eat(id);
    }
}

fn contemplate() {
    let mut rng: ThreadRng = rand::thread_rng();
    thread::sleep(Duration::from_millis(rng.gen_range(0..500)));
}

fn eat(id: u32) {
    if id == 3 {
        acquire_left_handed_fork(3);
    } else {
        acquire_right_handed_fork(id);
    }
    println!("Philosopher {} is eating.", id);
}

fn acquire_left_handed_fork(id: u32) {
    ARRAY.lock().unwrap()[id] = 33;
}

fn acquire_right_handed_fork(id: u32) {}

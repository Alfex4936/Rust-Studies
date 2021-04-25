fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    // Python unpack과 비슷
    println!("The value of y is: {}", y); // The value of y is: 6.4

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2; // index 접근은 .

    println!("{}, {}, {}", five_hundred, six_point_four, one); // 500, 6.4, 1
}

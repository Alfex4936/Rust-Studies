fn main() {
    // error[E0597]: `numbers` does not live long enough
    // let s;
    // {
    //     let numbers = [2, 4, 1, 0, 9];
    //     s = smallest_number(&numbers); // dangling pointer  as numbers will be gone
    // }
    // println!("{}", s)

    // error[E0597]: `y` does not live long enough
    // as r
    let x = 3;
    let r;
    {
        let y = 4;
        let point = Point { x: &x, y: &y };
        r = point.x
    }
    println!("{}", r);

    // Compiles successfully by differentiating lifetime
    let x = 3;
    let r;
    {
        let y = 4;
        let point = Point2 { x: &x, y: &y };
        r = point.x // 'b
    }
    println!("{}", r);

    // Compiles successfully
    let s;
    let numbers;
    let temp;
    {
        numbers = [2, 4, 1, 0, 9];
        s = smallest_number(&numbers); // dangling pointer  as numbers will be gone
        temp = numbers[2];
    }
    println!("{} and {}", s, temp)
}

struct Point<'a> {
    x: &'a i32,
    y: &'a i32,
}

struct Point2<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

// same as
// fn smallest_number<'a>(n: &'a [i32]) -> &'a i32 {
fn smallest_number(n: &[i32]) -> &i32 {
    let mut s = &n[0];
    for r in &n[1..] {
        if r < s {
            s = r;
        }
    }
    s
}

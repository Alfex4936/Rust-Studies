fn main() {
    let home1 = Home::new("rust".to_string(), None);
    let home2 = Home::new("python".to_string(), Some(5000));

    let mut homes: Vec<Home> = Vec::new();

    dbg!(&home1);
    dbg!(&home2);

    homes.push(home1);
    homes.push(home2);

    for home in &homes {
        println!(
            "{} Home: {} won",
            home.name,
            match home.price {
                None => "?".to_string(),
                Some(price) => price.to_string(),
            }
        );
    }

    // println!(
    //     "Rust Home: {} won",
    //     match home1.price {
    //         None => "unknown price".to_string(),
    //         Some(price) => price.to_string(),
    //     }
    // );

    // println!("Python Home: {} won", home2.price.unwrap_or(0));
}

#[derive(Debug)]
struct Home {
    name: String,
    price: Option<u32>,
}

impl Home {
    fn new(name: String, price: Option<u32>) -> Self {
        Home {
            name: name,
            price: price,
        }
    }
}

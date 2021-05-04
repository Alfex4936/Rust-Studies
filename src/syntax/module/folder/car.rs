pub struct Car {
    price: i32,
}

impl Car {
    pub fn new(price: i32) -> Self {
        Car { price: price }
    }

    pub fn get_price(&self) -> i32 {
        self.price
    }
}

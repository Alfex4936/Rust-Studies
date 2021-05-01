// A trait is a collection of methods defined for an unknown type: Self.
// They can access other methods declared in the same trait.

// The dyn keyword is used to highlight that calls to methods on the associated
// Trait are dynamically dispatched. To use the trait this way, it must be 'object safe'.

trait Vehicle {
    fn go(&self);
}

struct Car;
struct Motorcycle;

impl Vehicle for Car {
    fn go(&self) {
        println!("Hello, Car is moving!");
    }
}

impl Vehicle for Motorcycle {
    fn go(&self) {
        println!("Hello, Motorcycle is moving!");
    }
}

impl Car {
    fn new() -> Car {
        Car
    }
}

impl Motorcycle {
    fn new() -> Motorcycle {
        Motorcycle
    }
}

struct Shop {
    cars: Vec<Box<dyn Vehicle>>,
}

impl Shop {
    fn new() -> Shop {
        Shop { cars: Vec::new() }
    }

    fn add_car(&mut self, car: Box<dyn Vehicle>) {
        self.cars.push(car);
    }

    fn move_all(&self) {
        for car in &self.cars {
            car.go();
        }
    }
}

fn main() {
    let mut shop = Shop::new();
    shop.add_car(Box::new(Car::new()));
    shop.add_car(Box::new(Motorcycle::new()));

    shop.move_all();

    // let mut vec: Vec<Box<dyn Vehicle>> = vec![];
    // let car = Car::new();
    // let motor = Motorcycle::new();

    // vec.push(Box::new(car));
    // vec.push(Box::new(motor));

    // for vehicle in &vec {
    //     vehicle.go();
    // }
}

/*
    https://www.youtube.com/watch?v=T0Xfltu4h3A
    https://www.youtube.com/watch?v=m_phdVlkr6U
 */

pub fn main() {
    trait_intro()
}

trait Vehicle {
    fn go(&self) -> String;
    fn name(&self) -> String;
    fn wheels(&self) -> u8 {
        4
    }
}

struct Car {
    name: String,
}

impl Vehicle for Car {
    fn go(&self) -> String {
        format!("{} is driving", self.name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

struct Bike {
    name: String,
}

impl Bike {
    fn new(name: &str) -> Bike {
        Bike {
            name: name.to_string(),
        }
    }
}

impl Vehicle for Bike {
    fn go(&self) -> String {
        format!("{} is racing", self.name)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn wheels(&self) -> u8 {
        2
    }
}

fn trait_intro() {
    let car = Car {
        name: "Car".to_string(),
    };
    let bike = Bike::new("Motorbike");

    println!("{}", car.go());
    println!("{}", bike.go());
    println!("{} has {} wheels", car.name(), car.wheels());
    println!("{} has {} wheels", bike.name(), bike.wheels());
}
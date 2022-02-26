#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
// Corrected code: "mileage" u32 field removed, "age" tuple field added
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (String, u32),
}

#[allow(unused)]
#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

impl Car {
    pub fn new() -> Self {
        Self {
            color: String::from("red"),
            motor: Transmission::Automatic,
            roof: true,
            age: (String::from("male"), 20),
        }
    }
}

#[allow(unused)]
fn ok() {
    let _car = Car::new();
}

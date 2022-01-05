pub use sample::{Age, Car};

pub mod sample;

mod only_lib {
    use crate::Age;
    use crate::Car;

    fn car_factory() -> Car {
        return Car {
            color: "".to_string(),
            motor: Transmission::Manual,
            roof: false,
            age: (Age::New, 0),
        };
    }
}

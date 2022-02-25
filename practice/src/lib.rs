pub use sample::{Age, Car};

pub mod sample;
pub mod whatever;
mod common;

mod only_lib {

    use crate::Age;
    use crate::Car;
    use crate::sample::Transmission;

    fn car_factory() -> Car {
        return Car {
            color: "".to_string(),
            motor: Transmission::Manual,
            roof: false,
            age: (Age::New, 0),
        };
    }

    #[test]
    fn test_car_factory() {
        let _car = car_factory();
        println!("{:?}", _car);
    }
}

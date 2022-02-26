pub use sample::{Age, Car, Color};

mod common;
pub mod sample;
pub mod whatever;

mod only_lib {

    use crate::sample::Transmission;
    use crate::Age;
    use crate::Car;
    use crate::Color;

    #[allow(unused)]
    fn car_factory() -> Car {
        return Car {
            color: Color::Blue,
            motor: Transmission::Manual,
            roof: false,
            age: (Age::New, 0),
        };
    }
}

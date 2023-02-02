pub use sample::{Age, Car, Color};

pub mod client;
mod common;
mod demo;
mod deps;
pub mod exercise;
mod exercises;
pub mod thread_pool;
pub mod ui;

pub mod sample;
pub mod whatever;

pub use client::{Client, Connection};

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

use std::thread;
use std::time::Duration;

struct Catcher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Catcher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Catcher<T> {
        Catcher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
            Some(v) => v
        }
    }
}

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculate slowly...");

        thread::sleep(Duration::from_secs(1));
        num
    };

    expensive_closure(32);
}

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculate slowly...");

    thread::sleep(Duration::from_secs(1));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} push ups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} sit ups!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn version_up() {
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let _add_one_v3 = |x: u32| { x + 1 };
    let _add_one_v4 = |x: i32| x + 1;
}
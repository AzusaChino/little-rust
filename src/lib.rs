#![warn(clippy::all)]
#![allow(clippy::new_without_default)]
#![allow(clippy::unneeded_field_pattern)]
// 允许 unused code
#![allow(dead_code)]

// mod 引入与当前文件同级的文件夹下的文件
// crate 代表引用当前文件同级的文件
// super 代表当前文件的上一级目录

pub use crate::client::Client;
pub use crate::client::Connection;
pub use crate::demo::Person;
pub use crate::demo::Sex;
// Bringing a module into scope with use
pub use crate::front_of_house::hosting;

pub mod exercises;
pub mod demo;
pub mod exercise;
pub mod ui;
pub mod client;
pub mod thread_pool;

/// Adds one to the number given.
///
/// # Examples
///
/// ```rust
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub struct Button;

pub mod a {
    use super::Button;

    pub struct AdButton {
        pub b: Button,
    }
}

pub mod b {
    use crate::a::AdButton;
    use crate::Button;

    use self::c::CdButton;

    fn bd() {
        let _a = AdButton { b: Button };
        let _c = CdButton;
    }

    pub mod c {
        pub struct CdButton;
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // Calling a function using a relative path starting with super
        super::serve_order();
    }

    fn cook_order() {}
}

// crate for current folder
pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_wait_list();

    // relative
    front_of_house::hosting::add_to_wait_list();
}

pub fn eat_at_restaurant_2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant_3() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_4() {
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
    hosting::add_to_wait_list();
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}


impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
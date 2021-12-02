#![warn(clippy::all)]
#![allow(clippy::new_without_default)]
#![allow(clippy::unneeded_field_pattern)]

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
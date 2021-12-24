#![allow(dead_code)]
mod front_of_hourse {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_hourse::hosting::add_to_waitlist();

    front_of_hourse::hosting::add_to_waitlist();

    let mut meal = back_of_hourse::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.seasonal_fruit = String::from("blueberries");
}

fn server_order() {}

mod back_of_hourse {
    fn fix_incorrect_order() {
        cook_order();
        super::server_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
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
}
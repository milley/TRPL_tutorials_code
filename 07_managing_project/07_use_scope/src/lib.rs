#![allow(dead_code)]
mod front_of_hourse;

// use crate::front_of_hourse::hosting;
pub use crate::front_of_hourse::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
#![allow(dead_code)]
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
        None
    }
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator2<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        None
    }
}
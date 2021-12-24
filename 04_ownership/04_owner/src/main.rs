#![allow(unused_variables)]
fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World");

    println!("{}", s);

    let s1 = String::from("Hello World");
    let s2 = s1;
    // println!("{}", s1);

    // clone
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    let s = String::from("Hello World");
    take_ownership(s);
    let x = 5;
    makes_copy(x);
    println!("{}", x);


    let s1 = give_ownership();
    let s2 = s1;
    let s3 = takes_and_gives_back(s2);


    let s1 = String::from("Hello");
    let (s2, len)= calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn give_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

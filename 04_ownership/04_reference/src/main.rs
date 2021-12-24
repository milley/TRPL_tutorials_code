#![allow(unused_variables)]
fn main() {
    let s1 = String::from("Hello");
    let length = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, length);

    let mut s1 = String::from("Hello");
    let length = calculate_borrow_length(&mut s1);
    println!("The length of '{}' is {}.", s1, length);

    let mut s = String::from("Hello");
    {
        let s1 = &mut s;
    }
    let s2 = &mut s;

    
    // let mut s = String::from("Hello");
    // let r1 = &s;
    // let r2 = &s;
    // let s1 = &mut s;
    // println!("{} {} {}", r1, r2, s1);

    
    // let r = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_borrow_length(s: &mut String) -> usize {
    s.push_str(", World");
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

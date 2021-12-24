#![allow(unused_variables)]
fn main() {
    let guess: u32 = "42".parse().expect("Invalid number");

    println!("{}", guess);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 54 % 5;

    let t = true;
    let f: bool = false;

    let x = 'z';
    let y: char = 'ℤ';
    let z = '😻';
}

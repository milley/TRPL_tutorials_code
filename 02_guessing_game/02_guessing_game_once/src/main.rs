use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Gussing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret number is: {}", secret_number);

    println!("Please input a number:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Input error");

    // shadow
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("Your guess number is: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You win!!!!"),
        Ordering::Greater => println!("Too big!"),
    }
}

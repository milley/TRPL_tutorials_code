#![allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    another_func();
    another_function(5);

    let x = 5;
    let y = {
        let x = 1;
        // x + 3;  // 1) ()
        x + 3
    };

    // println!("y = {:?}", y);  // 1) tuple display
    println!("y = {}", y);

    let x = five();
    println!("x = {}", x);

    let z = add_five(6);
    println!("z = {}", z);
}

fn another_func() {
    println!("Another function!");
}

fn another_function(x: i32) {
    println!("The value of the x is: {}", x);
}

fn five() -> i32 {
    5
}

fn add_five(x: i32) -> i32 {
    x + 5
}

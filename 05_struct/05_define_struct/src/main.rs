#![allow(unused_variables, dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let user1 = User {
        email: String::from("abc@126.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 100,
    };

    let user2 = build_user(String::from("bob@gmail.com"), String::from("bob"));

    let user3 = User {
        email: String::from("kavin@126.com"),
        username: String::from("kavin"),
        ..user1
    };


    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    // area function
    // let w = 30;
    // let h = 50;
    // println!("{}", area(w, h));

    // let rect = (30, 50);
    // println!("{}", area(rect));

    let rect = Rectangle { width: 30, height: 50 };
    println!("{}", area(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 0,
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dim: (u32, u32)) -> u32 {
//     dim.0 * dim.1
// }

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

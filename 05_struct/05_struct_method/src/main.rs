#![allow(unused_variables, dead_code)]
use struct_method::Rectangle;


fn main() {
    let s = Rectangle::square(20);

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 10,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}

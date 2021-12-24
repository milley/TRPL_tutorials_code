
#![allow(unused_variables)]
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);


    // array
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let index = [12, 13, 14, 15, 16];
    // compile Ok, run Err
    // let month = months[index[1]];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b= [3; 5];
}

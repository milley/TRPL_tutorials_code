#![allow(unused_variables, unused_imports)]
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];


    let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("error opening file {:?}", error);
    //     }
    // };

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("error creating file {:?}", e),
    //         },
    //         other_error => panic!("error opening file {:?}", other_error),
    //     },
    // };


    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("error creating file: {:?}", error);
    //         })
    //     } else {
    //         panic!("error opening file: {:?}", error);
    //     }
    // });


    let f = File::open("hello.txt").expect("无法打开文件");
}

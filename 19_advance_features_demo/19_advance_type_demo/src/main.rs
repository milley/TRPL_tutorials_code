#![allow(dead_code, unused_variables)]
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}


use std::fmt;

type Result<T> = std::io::Result<T>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}


fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);


    let f: Thunk = Box::new(|| println!("hi"));

    // let guess = "";
    // loop {
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    // }
    

    let answer = do_twice(add_one, 5);
    println!("answer = {}", answer);



    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings : Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings : Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();


    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // println!("list_of_statuses: {:?}", list_of_statuses);


    let ret = returns_closure();
    println!("returns_closure: {:?}", ret(5));
}

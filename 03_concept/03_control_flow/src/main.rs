
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    let number: i32 = 6;
    match number {
        number if number % 4 == 0 => println!("number is divisible by 4"),
        number if number % 3 == 0 => println!("number is divisible by 3"),
        number if number % 2 == 0 => println!("number is divisible by 2"),
        _ => println!("number is not divisible by 4, 3, or 2"),
    }


    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);


    let mut cnt = 3;
    while cnt > 0 {
        println!("{}", cnt);
        cnt -= 1;
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

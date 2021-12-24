enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("{:?}", v);


    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in v {
        println!("{:?}", i);
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

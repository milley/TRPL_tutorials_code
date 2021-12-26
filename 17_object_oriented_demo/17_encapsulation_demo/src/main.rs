#![allow(dead_code)]
use encapsulation_demo::AveragedCollection;
use encapsulation_demo::Draw;
use encapsulation_demo::{Button, Screen};


fn print_average_collection(collect: &AveragedCollection) {
    println!("Average is: {}", collect.average());
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox drawing");
    }
}

fn main() {
    let mut averages = AveragedCollection::new();

    averages.add(1);
    print_average_collection(&averages);

    averages.add(2);
    averages.add(4);
    print_average_collection(&averages);

    averages.remove();
    print_average_collection(&averages);


    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            })
        ],
    };

    screen.run();
}

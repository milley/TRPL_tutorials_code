fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn largest_string<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> T {
//     let mut largest = list[0].clone();

//     for item in list.iter() {
//         if item > &largest {
//             largest = item.clone();
//         }
//     }

//     largest
// }

fn largest_string<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            largest = item;
        }
    }

    largest
}


// struct Point<T> {
//     x: T,
//     y: T,
// }



struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", largest(&char_list));

    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest_string(&str_list);
    println!("The largest string is {}", result);


    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };


    let p1 = Point { x: 5, y: 4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

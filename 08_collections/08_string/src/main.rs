fn main() {
    let data = "initial contents";
    let s = data.to_string();

    let s1 = "initial contents".to_string();
    let mut s2 = String::from("initial contents");
    s2.push_str(&s1);
    println!("{}", s2);


    let mut s = String::from("fo");
    s.push('o');
    println!("{}", s);


    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toc");
    // let s3 = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s3);
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);



    let len = String::from("Здравствуйте").len();
    println!("{}", len);


    let w = "नमस्ते";
    // for b in w.bytes() {
    //     println!("{}", b);
    // }
    for b in w.chars() {
        println!("{}", b);
    }
}

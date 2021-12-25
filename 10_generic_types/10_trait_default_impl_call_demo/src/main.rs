use trait_default_impl_call_demo::Summary;
use trait_default_impl_call_demo::Tweet;
use trait_default_impl_call_demo::Pair;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let pair = Pair::new(3, 5);
    pair.cmp_display();
}

// use export_with_pub_use::kinds::PrimaryColor;
// use export_with_pub_use::utils::mix;

use export_with_pub_use::PrimaryColor;
use export_with_pub_use::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

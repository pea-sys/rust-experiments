extern crate add_one;
extern crate add_two;

fn main() {
    let num = 10;
    // こんにちは世界！{}+1は{}!
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    // こんにちは世界！{}+1は{}!
    println!("Hello, world! {} plus two is {}!", num, add_two::add_two(num));
}
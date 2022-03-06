#[allow(unused_imports)]
use rand::prelude::*;

fn main() {
    println!("----------Consuming Crates----------");

    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("y = {}", y);

    let mut nums: Vec<i32> = (1..100).collect();
    println!("before shuffling nums = {:?}", nums);
    nums.shuffle(&mut rng);
    println!("after shuffling nums = {:?}", nums);
}

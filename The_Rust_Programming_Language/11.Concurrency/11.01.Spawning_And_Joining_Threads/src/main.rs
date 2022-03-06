#[allow(unused_imports)]
use std::{thread, time};

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    println!("--------Spawning And Joining Threads--------");

    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    handle.join();
}

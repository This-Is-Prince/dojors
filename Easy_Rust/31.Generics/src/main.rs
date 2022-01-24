/* ===========1========== */
/* fn return_number(number: i32) -> i32 {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
    println!("number is : {}", number);
}
 */

/* ===========2========== */
/* fn return_number<T>(number: T) -> T {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
    println!("number is : {}", number);
}
 */

/* ===========3========== */
/* fn return_number(number: T) -> T { // Error compiler think it is concrete type
    println!("Here is your number.");
    number
}
 */
/*
fn return_number<MyType>(number: MyType) -> MyType {
    println!("Here is your number.");
    number
}

fn main() {
    let number = return_number(5);
    println!("number is : {}", number);
}
 */

/* ===========4========== */
/* fn print_number<T>(number: T) {
    println!("Here is your number: {:?}", number); // Error compiler don't know , type implement Debug trait or not
}
 */
/* use std::fmt::Debug;
fn print_number<T: Debug>(number: T) {
    println!("Here is your number: {:?}", number);
}
fn main() {
    print_number(5);
}
 */

/* ===========5========== */
/* use std::fmt::Debug;

#[allow(unused)]
#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}
fn main() {
    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };
    let number = 55;
    print_item(charlie);
    print_item(number);
}
 */

/* ===========6========== */
/* use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}
fn main() {
    compare_and_display("Listen up!", 9, 8);
}
 */

/* ===========6========== */
use std::cmp::PartialOrd;
use std::fmt::Display;

fn compare_and_display<T, U>(statement: T, num_1: U, num_2: U) -> Result<bool, bool>
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
    if num_1 > num_2 {
        Ok(true)
    } else {
        Err(false)
    }
}
fn main() {
    compare_and_display("Listen up!", 9, 8);
}

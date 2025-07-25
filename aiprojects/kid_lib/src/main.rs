mod data;
mod logic;
mod utils;

use data::{Book, Color, Marker, Genre};
use logic::*;
use utils::print::line;

const MAX_BOOKS: usize = 100;

fn main() {
    line("=== Kid-Lib demo ===");

    let user_name = "Bob";
    let mut age: u8 = 10;
    age += 1;

    let fav = String::from("Rust in Action");

    println!("{user_name} is {age}. Loves {fav}");

    let rate: f32 = 4.5;
    let awesome: bool = true;
    let code: char = 'R';

    let tup: (u8, f32, bool) = (age, rate, awesome);
    let (a, r, _) = tup;
    println!("Tuple parts -> {a} and {r}");

    let grades = [100, 95, 80, 70];

    greet(user_name);

    check_grade(grades[0]);
    count_down(3);

    utils::ownership::run_all();

    let mut book = Book::new("Rust Book", Genre::Tech, 256);
    book.read_pages(30);
    println!("After reading: {:?}", book);

    let _marker = Marker;
    let magenta = Color(255, 0, 255);
    println!("RGB: {}-{}-{}", magenta.0, magenta.1, magenta.2);

    let cat = Genre::Fiction;
    describe_genre(cat);

    line("=== End ===");
}

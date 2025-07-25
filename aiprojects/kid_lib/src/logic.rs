use crate::utils::print::line;
use crate::data::Genre;

pub fn greet(name: &str) {
    line(&format!("Hello, {name}!"));
}

pub fn check_grade(score: i32) {
    if score >= 90 {
        println!("Great Job!");
    } else if score >= 75 {
        println!("Good.");
    } else {
        println!("Keep trying.");
    }
}

pub fn count_down(mut n: i32) {
    while n > 0 {
        println!("{n}...");
        n -= 1;
    }

    let mut i = 0;
    loop {
        i += 1;
        if i == 2 { break; }
    }

    for x in 1..=3 {
        print!("{x} ");
    }
    println!();
}

pub fn describe_genre(g: Genre) {
    match g {
        Genre::Fiction => println!("It's a story"),
        Genre::Tech => println!("It's about computers."),
        Genre::Comics => println!("It has pictures."),
    }

    if let Genre::Tech = g {
        println!("Nerdy Choice!");
    } else {
        println!("Non-tech choice!");
    }
}
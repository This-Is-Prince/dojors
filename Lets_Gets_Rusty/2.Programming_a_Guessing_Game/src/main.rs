use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please enter your guess : ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim().parse::<u32>() {
                Ok(guess) => match guess.cmp(&secret_number) {
                    Ordering::Less => {
                        println!("{}", "Too Small!".red());
                    }
                    Ordering::Greater => {
                        println!("{}", "Too Big!".red())
                    }
                    Ordering::Equal => {
                        println!("{}", "You Win!".green());
                        break;
                    }
                },
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }
}

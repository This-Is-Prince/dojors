use rand::Rng;
use std::io::stdin;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter your guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!!!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e);
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}

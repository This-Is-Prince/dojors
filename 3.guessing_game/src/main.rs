// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("<----- Guessing Game ----->");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     loop {
//         println!("Enter a guess:");
//         // let secret_number = rand::thread_rng().gen_range(1..101);
//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to get input");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too Low!"),
//             Ordering::Greater => println!("Too High!"),
//             Ordering::Equal => {
//                 println!("You Win!");
//                 break;
//             }
//         }
//     }
// }

// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

// fn main() {
//     println!("<--- Guessing Game --->");
//     let secret_number = rand::thread_rng().gen_range(1..101);
//     loop {
//         println!("Enter a guess : ");
//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to get input");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too low!"),
//             Ordering::Greater => println!("Too High!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("<--- Welcome , let's play guessing game --->");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please enter your guess :-> ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        };
    }
}

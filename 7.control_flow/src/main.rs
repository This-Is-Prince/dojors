// fn main() {
//     let n = 6;
//     let m = 7;
//     if n < m {
//         println!("n is less than {}", m);
//     } else {
//         println!("n is greater than {}", m);
//     }
// }

// use std::io;

// fn main() {
//     let mut input = String::new();
//     loop {
//         println!("Please enter a number: ");
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to get input");
//         let input_number: i32 = match input.trim().parse() {
//             Ok(num) => num,
//             Err(err) => {
//                 println!("Error is: {}", err);
//                 continue;
//             }
//         };
//         input = String::new();
//         if input_number % 2 == 0 {
//             println!("Input is Even");
//         } else {
//             println!("Input is Odd");
//         }
//     }
// }

// fn main() {
//     let n = 5;
//     if n < 10 {
//         println!("N is less than 10")
//     } else if n > 10 {
//         println!("N is greater than 10")
//     } else {
//         println!("N is equal to 10")
//     }
// }

fn main() {
    let n = 5;
    let a = if n < 10 { n } else { -1 };
    println!("The value of a is : {}", a);
}

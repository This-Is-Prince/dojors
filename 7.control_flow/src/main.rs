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

// fn main() {
//     let n = 5;
//     let a = if n < 10 { n } else { -1 };
//     println!("The value of a is : {}", a);
// }

// fn main() {
//     // Infinite loop
//     loop {
//         println!("Again!");
//     }
// }

// fn main() {
//     let mut x = 1;
//     loop {
//         println!("Again! {}", x);
//         x = x + 1;
//         if x == 100 {
//             break;
//         }
//     }
//     println!("The value of x is : {}", x);
// }

// fn main() {
//     let mut x = 1;
//     'outer_loop: loop {
//         println!("The value of x in outer loop : {}", x);
//         loop {
//             if x >= 100 {
//                 break 'outer_loop;
//             } else {
//                 x = x + 1;
//                 println!("The value of x in inner loop : {}", x);
//             }
//         }
//     }
// }

// fn main() {
//     let mut counter = 0;
//     let value = loop {
//         counter = counter + 1;
//         if counter == 100 {
//             break counter * 2;
//         }
//     };
//     println!("The value of value is : {}", value);
// }

// fn main() {
//     let mut number = 10;
//     while number != 0 {
//         println!("The value of number is {}", number);
//         number -= 1;
//     }
//     println!("The end!");
// }

// fn main() {
//     let arr = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while index < arr.len() {
//         println!("The value at {} index is : {}", index, arr[index]);
//         index += 1;
//     }
//     println!("The End!");
// }

// fn main() {
//     let arr = [10, 20, 30, 40, 50];
//     for element in arr {
//         println!("The value is : {}", element);
//     }
// }

fn main() {
    let a = (1..10).rev();
    for num in a {
        println!("The arr value : {}", num);
    }
    for number in (1..4).rev() {
        println!("{}", number)
    }
}

fn main() {
    // Can't assign x twice because x is immutable
    // let x=5;
    // println!("The value of x is : {}",x);
    // x = 6;
    // println!("The value of x is : {}",x);

    // mut keyword for mutable variables
    // let mut x = 5;
    // println!("The value of x is : {}", x);
    // x = 6;
    // println!("The value of x is : {}", x);

    // Const variable
    // const SUBSCRIBER_COUNT: u32 = 100_000;
    // println!("The value of SUBSCRIBER_COUNT is {}", SUBSCRIBER_COUNT);

    // Shadowing
    // let x = 5;
    // println!("The value of x is : {}", x);
    // let x = "six";
    // println!("The value of x is : {}", x);

    // Data Types
    // let a: i32 = 98_222;

    // let f: u8 = 255;

    // let e: f64 = 2.2;

    // let t: bool = true;

    // Character
    // let c: char = 'z';

    // TUple
    // let tup: (&str, i32) = ("Let's Get Rusty!", 100_000);
    // // println!("The value of tup.0 is {}", tup.0);
    // // println!("The value of tup.1 is {}", tup.1);
    // let (channel, sub_count) = tup;
    // println!("The value of channel is {}", channel);
    // println!("The value of sub_count is {}", sub_count);

    // Arrays
    // let error_codes: [i32; 3] = [200, 404, 500];
    // let not_found = error_codes[1];
    // let byte = [0; 8];

    // FUnctions
    // my_function()
    // my_function2(1, 2);
    // println!("The value of return is {}", my_function_return(1, 2));
    // let number = 5;
    // if number < 5 {
    //     println!("number is less than 5");
    // } else if number < 10 {
    //     println!("number is less than 10");
    // } else {
    //     println!("number is greater than 10");
    // }

    // let condition = true;
    // let number = if condition { 1 } else { 0 };
    // println!("The number is {}", number);

    // Loops
    // loop {
    //     println!("again!")
    // }

    // let mut counter = 0;
    // loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break;
    //     } else {
    //         continue;
    //     }
    // }
    // println!("The value of counter is {}", counter);

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter;
    //     } else {
    //         continue;
    //     }
    // };
    // println!("The value of counter is {}", counter);
    // println!("The value of result is {}", result);

    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!")

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("The value is: {}", number);
    }
    /*
    This is multiline comment
    */
    // This is single line comment
}

// fn my_function() {
//     println!("Another Function.");
// }
// fn my_function2(x: i32, y: i32) {
//     println!("The value of x is : {}", x);
//     println!("The value of y is : {}", y);
// }
// fn my_function_return(x: i32, y: i32) -> i32 {
//     println!("The value of x is : {}", x);
//     println!("The value of y is : {}", y);
//     // return x + y;
//     x + y
// }

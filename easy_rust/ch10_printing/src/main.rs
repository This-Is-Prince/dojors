fn main() {
    println!("Printing `hello, world!`");

    println!("Hello, worlds number {}!", 8);
    println!("Hello, worlds number {} and {}!", 8, 9);
    println!("Hello, worlds number {}!", number());

    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);

    let multiply1_result = multiply1(8, 9);
    println!("multiply1 result is {}", multiply1_result);

    // let my_number = 8;
    // println!("Hello, number {}", my_number);

    // {
    //     let my_number = 8;
    //     println!("my_number {}", my_number);
    // }
    // println!("Hello, number {}", my_number); // ERROR, cannot find value `my_number` in this scope

    let my_number = {
        let second_number = 8;
        second_number + 9
    };
    println!("My number is: {}", my_number);

    #[allow(unused_must_use)]
    let my_number = {
        let second_number = 8;
        second_number + 9;
    };
    println!("My number is: {:?}", my_number);
}

// fn number() -> i32 {
//     8 // ERROR, expected `i32`, found `()`
// }

fn number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

fn multiply1(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
    result
}

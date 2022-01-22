fn number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} time {} is {}", number_one, number_two, result);
    result
}

#[allow(unused_must_use)]
fn main() {
    println!("Hello, world number {}!", 8);
    println!("Hello, world number {} and {}!", 8, 9);
    println!("Hello, world number {}!", number());

    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);
    let multiply_result = multiply(4, 5);
    println!("multiply result is {}", multiply_result);

    let my_number = 9;
    println!("Hello, number {}", my_number);

    /*
    {
        let hi_number = 8;
    }
    println!("Hello, number {}", hi_number); // Error hi_number is not valid
    */

    let number = {
        let second_number = 9;
        second_number + 9
    };
    println!("My number is: {}", number); // 18

    let number = {
        let second_number = 8;
        second_number + 1;
    };
    println!("My number is: {:?}", number); // ()
}

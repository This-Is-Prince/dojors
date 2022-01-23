#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    /*
    ==============
    Mutability
    ==============
    */
    /*
    let my_number = 8;
    my_number=10; // Error
    */

    /*
    let mut my_number = 9;
    my_number = 10;
    */

    /*
    let mut my_variable=8;
    my_variable="Hello, world!"; // Error
    */

    /*
    ==============
    Shadowing
    ==============
    */
    /*
    let my_number = 8;
    println!("{}", my_number);
    let my_number = 9.2;
    println!("{}", my_number);
    */

    /*
    let my_number = 9;
    println!("{}", my_number);
    {
        let my_number = 9.2;
        println!("{}", my_number);
    }
    println!("{}", my_number);
    */

    /*
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x
    };
    println!("The number is now: {}", final_number);
    */

    let final_number = {
        let y = 10;
        let x = 9;
        let x_twice = times_two(x);
        let x_twice_and_y = x_twice + y;
        x_twice_and_y
    };
    println!("The number is now: {}", final_number);
}

fn times_two(number: i32) -> i32 {
    number * 2
}

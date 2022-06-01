fn main() {
    println!("Tutorial 3 - Variables, Constants and Shadowing.");
    // let x = 4;
    // println!("x is: {}", x);

    // x = 5; // ERROR, cannot assign twice to immutable variable `x`
    // println!("x is: {}", x);

    // let mut x = 4;
    // println!("x is: {}", x);

    // x = 5; // ERROR, cannot assign twice to immutable variable `x`
    // println!("x is: {}", x);

    // let x = 4;
    // println!("x is: {}", x);
    // let x = 5;
    // println!("x is: {}", x);

    // let x = 4;
    // println!("x is: {}", x);

    // {
    //     let x = x - 2;
    //     println!("x is: {}", x);
    // }

    // let x = x + 1;
    // println!("x is: {}", x);

    let x = 4;
    println!("x is: {}", x);

    let x = "hello";
    println!("x is: {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("SECONDS_IN_MINUTE is: {}", SECONDS_IN_MINUTE);

    // const SECONDS_IN_MINUTE: u32 = 60; // ERROR, the name `SECONDS_IN_MINUTE` is defined multiple times
    // println!("SECONDS_IN_MINUTE is: {}", SECONDS_IN_MINUTE);
}

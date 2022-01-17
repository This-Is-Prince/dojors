fn main() {
    /*     // variable are immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants cannot be mutable
    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);
    */

    /*// -----------Shadowing-----------
    let x: i32 = 5;
    println!("The value of x is: {}", x);
    let x: &str = "six";
    println!("The value of x is: {}", x);
    */

    /* // -----------Data Types-----------
    // 1 -> Scaler Data Types
    // Integers
    let _a: i32 = 98_222; // Decimal
    let _b: i32 = 0xff; // Hex
    let _c: i32 = 0o77; // Octal
    let _d: i32 = 0b111_0000; // Binary
    let _e: u8 = b'A'; // Byte (u8 only)
    let _f: u8 = 255; // 256 Integer overflow

    // Floating-point numbers
    let _f: f64 = 2.1;
    let _g: f32 = 3.2;

    // Booleans
    let _t = true; // true/false

    // Character
    let _c: char = 'z';
    let _heart_eyed_cat: char = '😍';

    // 2 -> Compound Types
    // Tuples
    let tup: (&str, i32, f64) = ("Prince", 100_000, 2.3);
    let (_name, _number, _float) = tup;
    let _number = tup.1;

    // Arrays
    let error_codes: [u16; 3] = [200, 404, 500];
    let _not_found: u16 = error_codes[1];
    let _byte: [u8; 8] = [0; 8];
    */

    /*     // -----------Functions-----------
    println!("sum of 1 and 2 is: {}", my_function(1, 2));
     */

    // -----------Control Flow-----------
    let number = 5;
    if number == 5 {
        println!("number is equal to 5");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is greater than 5")
    }

    let mut x = 1;
    let result = loop {
        println!("again {}", x);
        x += 1;
        if x == 11 {
            break x;
        }
    };
    println!("result is {}", result);
    println!("x is {}", x);
    println!("x==result {}", x == result);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("the number is: {}", number);
    }
}
/* fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    x + y
}
 */

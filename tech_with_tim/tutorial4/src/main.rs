fn main() {
    println!("Tutorial 4 - Data Types");

    // Scalar Data Types
    // 1. Integer
    // u8, u16, u32, u64, u128
    // i8, i16, i32, i64, i128
    // 2. Floating
    // f32, f64
    // 3. Character
    // char
    // 4. Boolean
    // bool
    let x = 2; // x is i32 by default
    println!("x is: {}", x);
    let x: i32 = 2; // we can also tell which type it is
    println!("x is: {}", x);

    let x: i128 = 44;
    println!("x is: {}", x);

    let x: char = 'a';
    println!("x is: {}", x);

    let x: bool = false;
    println!("x is: {}", x);

    let x = 64.44; // f64 by default
    println!("x is: {}", x);

    let x: f32 = 64.44;
    println!("x is: {}", x);

    // Compound Data Types
    // 1. Tuple
    let tup = (5, 5.4, true, 's', "Hello", String::from("Hello World!"));
    println!("tup is: {:?}", tup);

    let tup1: (i32, bool, char) = (1, true, 's');
    println!("tup1 is: {:?}", tup1);

    let tup2: (i8, bool, char) = (1, true, 's');
    println!("tup2 is: {:?}", tup2);
    // tup1 is not equal to tup2

    let mut tup: (i32, bool, char) = (1, true, 's');
    tup.0 = 10;
    println!("tup.0 {}", tup.0);
    tup = (100, false, 'e');
    println!("tup {:?}", tup);

    // 2. Array
    let arr = [1, 2, 3, 4, 5]; // data type is:- [i32; 5]
    println!("arr is: {:?}", arr);

    let mut arr = [1, 2, 3, 4, 5];
    println!("arr is: {:?}", arr);
    arr[4] = 400;
    println!("arr is: {:?}", arr);

    let x: u8 = 4;
    // let y:i32=x; // ERROR, mismatched types
    let y: i32 = x.into();
    println!("{}, {}", x, y);
}

use std::io;
fn main() {
    println!("Tutorial 5 - Arithmetic and Type Casting");

    let x: u8 = 240; // 0 - 255
    let y: i8 = 10; // -128 - 127
    let z = x + y as u8;
    println!("z is: {}", z);

    println!("{}", i32::MAX);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("expected to read line");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input);
}

fn main() {
    println!("Type Inference");

    let small_number: u8 = 10;
    println!("{}", small_number);
    let small_number = 10u8; // 10u8 = 10 of type u8
    println!("{}", small_number);
    let small_number = 10_u8; // This is easier to read
    println!("{}", small_number);
    let big_number = 100_000_000_i32; // 100 million is easy to read with _
    println!("{}", big_number);
    let number = 0_____________________u8;
    println!("{}", number);
    let number2 = 1__6____2______4_______i32;
    println!("{}", number2);

    // Floats
    let my_float = 5.;
    println!("{}", my_float);
    let my_float: f64 = 5.0; // This is an f64
    println!("{}", my_float);
    let my_other_float: f32 = 8.5; // This is an f32
    println!("{}", my_other_float);
    // let third_float = my_float + my_other_float;  // ERROR, expected `f64`, found `f32`
    let third_float = my_float + my_other_float as f64;
    println!("{}", third_float);

    let my_float = 5.0; // Rust will choose f64
    let my_other_float = 8.5; // Here again it will choose f64

    let third_float = my_float + my_other_float;
    println!("{}", third_float);

    let my_float: f32 = 5.0;
    let my_other_float = 8.5;
    let third_float = my_float + my_other_float;
    println!("{}", third_float);
}

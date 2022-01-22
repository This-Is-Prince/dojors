#[allow(unused_variables)]
fn main() {
    // Integer
    // Signed i8,i16,i32,i64,i128
    // UnSigned u8,u16,u32,u64,u128

    let small_number: u8 = 10;
    let small_number = 10u8;
    let small_number = 10_u8;
    let large_number = 100_000_000_i64;
    let large_number = 100_000_000_000i128;
    let large_number = 100_000_000_000_000_i128;

    let number = 0_______u8;
    let number2 = 1__6____2___4___u64;
    println!("{}, {}", number, number2);

    // Float by default f64
    // f32,f64
    let my_float = 5.;
    let my_float = 5.5;
    let my_float = 5.0;

    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;
    // let third_float = my_float + my_other_float; // Error
    let third_float = my_float + my_other_float as f64;

    let my_float = 5.0;
    let my_other_float = 9.5;
    let third_float = my_float + my_other_float;
    println!("{}", third_float);

    let float1: f32 = 5.0;
    let float2 = 9.9;
    let float_sum = float1 + float2;
    println!("{}", float_sum);
    let float_sum = float2 + float1;
    println!("{}", float_sum);
}

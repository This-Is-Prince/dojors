#[allow(unused_variables)] // Allow unused Variables
#[allow(unused_assignments)] // Allow Unused Assignments

fn main() {
    // let some_data: bool = true; // or false
    // let some_data = false; // compiler can infer data types
    // let mut some_data = false;
    // some_data = true;

    // let mut some_data: i8 = 10;
    // some_data = 127; // i8 integer range from -128 to 127
    // let test = some_data + 120; // Error
    // println!("MIN i8 is {}", std::i8::MIN);
    // println!("MAX i8 is {}", std::i8::MAX);

    // let some_data: u8 = 10; // from 0 to 255
    // println!("MIN u8 is {}", std::u8::MIN);
    // println!("MAX u8 is {}", std::u8::MAX);

    // i8, i16, i32, i64, i128
    // u8, u16, u32, u64, u128
    // println!("MIN u128 is {}", std::u128::MIN);
    // println!("MAX u128 is {}", std::u128::MAX);

    // let some_isize: isize = 10; // depends on what ever architecture of computer, 32bit or 64bit
    // let some_isize: usize = 10; // depends on what ever architecture of computer, 32bit or 64bit
    // println!("MIN isize is {}", std::isize::MIN);
    // println!("MAX isize is {}", std::isize::MAX);

    // Float
    // f32, f64
    // let some_data: f32 = 10.; // dot is necessary in float number
    // let some_data = 10.2; // by default float number is f64 types

    // Char
    // char
    let some_char: char = 'a';
    println!("MAX char is {}", std::char::MAX);
    println!("char is {} bytes", std::mem::size_of_val(&some_char));
    println!("some_char value is : {}", some_char);
}

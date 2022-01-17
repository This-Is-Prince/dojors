#[allow(unused_variables)] // prevent from warning related to unused variables.
#[allow(unused_assignments)] // prevent from warning related to unused assignments.

// variable and function name written acc to snake case
fn main() {
    // #[allow(non_snake_case)] // prevent from warning related to non snake case variable or functions name.
    // let Some_data: bool = true; // or false
    // let some_data: bool = true; // or false
    // let some_data = true;
    // by default variable are immutable in rust
    // some_data=false;// error because some_data is not immutable if we want to mutate it we must use mut keyword in front of some_data variable at the time of creation of variable

    // unsigned integer u8 = 2 ^ N = 2 ^ 8. = 256
    // signed integer i8 = 2 ^ (N - 1) - 1 = 2 ^ 7 - 1. = 128
    // let some_data: u8 = 10; // 0 to 255
    // let some_data: i8 = 10; // -128 to +127
    // println!("Min i8 is {}", std::i8::MIN);
    // println!("Max i8 is {}", std::i8::MAX);

    // let test = some_data + 120; // Panic integer overflow
    // println!("test is {}", test);

    // let some_data: u8 = 10; // from 0 to 255
    // let some_other_data = 255 + some_data; // Error!!! Too big
    // let some_other_data2 = 1 - some_data; // Error!!! Too small

    // let some_data: i128 = 10; // from -WOW to +WOW - 1
    // println!("Min i128 is {}", std::i128::MIN);
    // println!("Max i128 is {}", std::i128::MAX);

    // let some_isize: isize = 10; //depends on whether your computer is 32bit or 64bit
    // let some_usize: usize = 10; //same here

    // let some_data: f32 = 10.; //Don't forget the dot somewhere in the computer
    // println!("some_isize is {}", some_isize);
    // println!("some_usize is {}", some_usize);
    // println!("some_data is {}", some_data);

    let some_char: char = 'a';
    println!("some_char is {}", some_char)
}

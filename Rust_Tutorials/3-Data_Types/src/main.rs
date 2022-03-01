#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // by default variable are immutable in rust
    // let some_data: bool = true; //or false
    // let some_data = false;

    // mutable variable
    // let mut some_data=true;
    // some_data=false;

    // let some_data: i8 = 1000; // error, from -128 to +127
    // println!("Min i8 is {}", std::i8::MIN);
    // println!("Max i8 is {}", std::i8::MAX);

    // let some_data: i8 = 10;
    // let prince_test = some_data + 117; // no error
    // let prince_test = some_data + 118; // error

    // let some_data: u8 = 10; // from 0 to 255
    // let some_data: u8 = -10; // error
    // let some_other_data = 255 + some_data; // ERROR!!!! Too Big
    // let some_other_data = 1 - some_data; // ERROR!!!! Too Small

    // let some_data: i128 = 10; // from -WOW to +WOW -1
    // println!("Min i128 is {}", std::i128::MIN);
    // println!("Max i128 is {}", std::i128::MAX);

    // let some_isize: isize = 10; // depends on whether your computer is 32bit or 64bit
    // let some_usize: usize = 10; // same here

    // let some_data: f32 = 10.; // Don't forget the dot somewhere in the number

    let some_char: char = 'a'; // More than just ascii!!
}

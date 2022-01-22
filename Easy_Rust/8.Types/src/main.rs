use std::*;

#[allow(unused_variables)]
fn main() {
    /*
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '😺';
    println!(
        "{}, {}, {}, {}",
        first_letter, space, other_language_char, cat_face
    );
    */

    /*
    let my_number = 100;
    println!("{}", my_number as char); // Error
    */

    /*
    let my_number: u32 = 100;
    println!("{}",my_number as char); // Error
    */

    /*
    let my_number: u8 = 250;
    println!("{}", my_number as char);

    for i in 0..=255 {
        print!("{} ", i as u8 as char);
    }
    */

    println!("Size of a char: {}", mem::size_of::<char>());
    println!("Size of string containing `a`: {}", "a".len());
    println!("Size of string containing `ß`: {}", "ß".len());
    println!("Size of string containing `国`: {}", "国".len());
    println!("Size of string containing `𓅱`: {}", "𓅱".len());
    println!("Size of string containing `国`: {}", mem::size_of_val("国"));
    println!("Size of string containing `𓅱`: {}", mem::size_of_val("𓅱"));

    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes.", slice2.len());

    let slice = "Hello!";
    println!(
        "Slice is {} bytes and also {} characters.",
        slice.len(),
        slice.chars().count()
    );
    let slice2 = "안녕!";
    println!(
        "Slice2 is {} bytes but only {} characters.",
        slice2.len(),
        slice2.chars().count()
    );

    println!("1");
    for (i, ch) in slice.chars().enumerate() {
        println!("{}:{}", i, ch);
    }
    println!("2");
    for (i, ch) in slice2.chars().enumerate() {
        println!("{}:{}", i, ch);
    }
}

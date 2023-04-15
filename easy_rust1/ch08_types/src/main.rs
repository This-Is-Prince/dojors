#[allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    let first_letter = 'A';
    let cat_face = '😻';
    let space = ' ';
    let other_language_char = '你';

    let my_number: u8 = 100;
    println!("{}", my_number as char);
    println!("Size of char: {}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes and also {} characters.", slice2.len(), slice2.chars().count());
}

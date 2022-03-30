fn main() {
    println!("Types");

    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '😺';
    println!(
        "{} {} {} {}",
        first_letter, space, other_language_char, cat_face
    );

    let my_number = 100;
    // println!("{}", my_number as char) // ERROR, only `u8` can be cast as `char`, not `i32`
    println!("{}", my_number as u8 as char);

    let my_number1: u8 = 100;
    println!("{} {}", my_number1, my_number1 as char);

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of a string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of a string containing 'ß': {}", "ß".len());
    println!("Size of a string containing '国': {}", "国".len());
    println!("Size of a string containing '𓅱': {}", "𓅱".len());

    let slice = "Hello!";
    println!(
        "Slice is {} bytes and also {} characters",
        slice.len(),
        slice.chars().count()
    );
    let slice2 = "안녕!";
    println!(
        "Slice2 is {} bytes and also {} characters",
        slice2.len(),
        slice2.chars().count()
    );
}

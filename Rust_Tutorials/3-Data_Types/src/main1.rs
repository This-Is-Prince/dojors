#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
pub fn main1() {
    // by default variables are immutable in rust
    let some_data: bool = true; // or false
                                // some_data = false; // error, cannot assign twice to immutable variable `some_data`
    println!("{}", some_data);
    let some_data = false;
    println!("{}", some_data);
    let some_data: i8 = 0;
    println!("{}", some_data);

    // declaring mutable variable
    let mut some_mut_data: bool = true;
    println!(
        "size of bool variable is {}bytes",
        std::mem::size_of_val(&some_mut_data)
    );
    if some_mut_data {
        some_mut_data = !some_mut_data;
        println!("some_mut_data: {}", some_mut_data)
    } else {
        println!("some_mut_data: {}", some_mut_data)
    }

    // Integer Types, by default variable have type f64
    // Signed Integer (i8, i16, i32, i64, i128)
    min_max_of_signed_integer();
    // UnSigned Integer (u8, u16, u32, u64, u128)
    min_max_of_unsigned_integer();

    // Architecture Dependent Types
    // isize, usize
    let isize_var: isize = -50000000000;
    let usize_var: usize = 50000000000;
    println!("isize_var: {}", isize_var);
    println!("usize_var: {}", usize_var);

    // Floating Points Types
    // 2 Types only (f32, f64) by default variable have type f64
    let f32_var: f32 = 1.;
    let f64_var: f64 = 2.;
    let f64_by_default_var = 10.;
    println!("f32_var: {}", f32_var);
    println!("f64_var: {}", f64_var);
    println!("f64_by_default_var: {}", f64_by_default_var);
    println!(
        "size of f32 variable is {}bytes",
        std::mem::size_of_val(&f32_var)
    );
    println!(
        "size of f64 variable is {}bytes",
        std::mem::size_of_val(&f64_var)
    );

    // Character Types
    let char_var1: char = 'a';
    // let char_var2: char = "a"; // Error, (expected `char`, found `&str`), single quotes around character
    println!(
        "size of char variable is {}bytes",
        std::mem::size_of_val(&char_var1)
    );
}

fn min_max_of_signed_integer() {
    println!("Min Max of Signed Integer");
    println!("Min i8 is {}", std::i8::MIN);
    println!("Max i8 is {}", std::i8::MAX);
    println!("Min i16 is {}", std::i16::MIN);
    println!("Max i16 is {}", std::i16::MAX);
    println!("Min i32 is {}", std::i32::MIN);
    println!("Max i32 is {}", std::i32::MAX);
    println!("Min i64 is {}", std::i64::MIN);
    println!("Max i64 is {}", std::i64::MAX);
    println!("Min i128 is {}", std::i128::MIN);
    println!("Max i128 is {}", std::i128::MAX);
}

fn min_max_of_unsigned_integer() {
    println!("Min Max of UnSigned Integer");
    println!("Min u8 is {}", std::u8::MIN);
    println!("Max u8 is {}", std::u8::MAX);
    println!("Min u16 is {}", std::u16::MIN);
    println!("Max u16 is {}", std::u16::MAX);
    println!("Min u32 is {}", std::u32::MIN);
    println!("Max u32 is {}", std::u32::MAX);
    println!("Min u64 is {}", std::u64::MIN);
    println!("Max u64 is {}", std::u64::MAX);
    println!("Min u128 is {}", std::u128::MIN);
    println!("Max u128 is {}", std::u128::MAX);
}

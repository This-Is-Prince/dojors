fn main() {
    let x = 4;
    println!("x is :{}", x);
    let x: u32 = 5;
    println!("x is :{}", x);

    // x=5; //
    let mut x: u32 = 6;
    println!("x is :{}", x);
    x = 76;
    println!("x is :{}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE)
}

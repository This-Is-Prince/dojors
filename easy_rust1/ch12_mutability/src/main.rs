fn main() {
    println!("Hello, world!");
    let mut a = 10;
    println!("a = {}", a);
    a = 20;
    println!("a = {}", a);

    let b = 10;
    let ptr = &b;
    let b = 20;
    println!("{}, {:p}", b, ptr);
}

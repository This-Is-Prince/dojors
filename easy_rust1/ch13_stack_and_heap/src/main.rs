fn main() {
    println!("Hello, world!");
    let a = 8;
    let reference = &a;
    println!("{}, {}, {}", a, reference, a == *reference);
}

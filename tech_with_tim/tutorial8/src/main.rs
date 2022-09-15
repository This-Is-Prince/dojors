fn main() {
    println!("Function In Rust");

    test_1();
    println!("x + y : {}", add_two_number(1, 2))
}

fn test_1() {
    println!("This is test1")
}

fn add_two_number(x: i32, y: i32) -> i32 {
    x + y
}

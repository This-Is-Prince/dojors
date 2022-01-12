fn main() {
    print_hello();
    let x = add_two_number(2, 3);
    let mut y = 5;
    println!("The value of x and y is: {},{}", x, y);
    y = add_two_number(5, 6);
    let z = {
        println!("The value of x and y is: {},{}", x, y);
        // x+y;//Error because its a statement (;)
        x + y // its a expression
    };
    println!("The value of z is : {}", z);
}
fn print_hello() {
    println!("Hello, world!");
}
fn add_two_number(first: i32, second: i32) -> i32 {
    // return first + second;
    first + second
}

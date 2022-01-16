fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    println!("increase x = {}", x);
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn functions() {
    print_value(33);

    let mut z = 1;
    println!("z = {}", z);
    increase(&mut z);
    println!("z = {}", z);

    println!("3 * 4 = {}", product(3, 4));
}

fn main() {
    functions();
}

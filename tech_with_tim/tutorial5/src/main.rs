use std::io;

fn main() {
    println!("Tutorial 5 - Console Input");
    let mut input = String::new();

    if let Ok(value) = io::stdin().read_line(&mut input) {
        println!("total no of bytes read: {}", value);
        println!("input value is: {}", input);
    } else {
        println!("Failed to read line")
    }

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("input value is: {}", input);
}

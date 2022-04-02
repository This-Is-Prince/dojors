#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    println!("Mutability (Changing)");

    let my_number = 8;
    // my_number = 10; // ERROR, cannot assign twice to immutable variable `my_number`

    let mut my_number = 8;
    my_number = 10;
    println!("{}", my_number);

    let mut my_variable = 8;
    // my_variable = "Hello, World!" ; // ERROR, expected integer, found `&str`

    // SHADOWING
    let my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    let my_number = 9.2; // This is an f64 with the same name. But it's not the first my_number - it is completely different!
    println!("{}", my_number); // Prints 9.2

    let my_number = 8; // This is an i32
    println!("{}", my_number); // prints 8
    {
        let my_number = 9.2; // This is an f64 with the same name. But it's not the first my_number - it is completely different!
        println!("{}", my_number); // Prints 9.2
                                   // But the shadowed my_number only live until here.
                                   // The first my_number is still alice!
    }
    println!("{}", my_number); // prints 8
}

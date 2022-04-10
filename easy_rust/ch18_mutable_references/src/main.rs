#[allow(unused_variables)]
fn main() {
    println!("Mutable References");
    let mut my_number = 8; // don't forget to write mut here!
    let num_ref = &mut my_number;
    *num_ref += 10; // Use * to change the i32 value.
    println!("{}", my_number);

    let second_number = 800;
    let triple_reference = &&&second_number;
    println!(
        "Second_number = triple_reference? {}",
        second_number == ***triple_reference
    );
    rules();
    shadowing();
}

fn rules() {
    /*
    let mut number = 10;
    let number_ref = &number;
    let number_change = &mut number; // ERROR, cannot borrow `number` as mutable because it is also borrowed as immutable
    *number_change += 10;
    println!("{}", number_ref);
     */

    let mut number = 10;
    let number_change = &mut number; // create a mutable reference
    *number_change += 10; // use mutable reference to add 10

    let number_ref = &number; // create an immutable reference
    println!("{}", number_ref); // print the immutable reference
}

fn shadowing() {
    let country = String::from("Australia");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}

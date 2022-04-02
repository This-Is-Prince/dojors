fn main() {
    println!("The Stack, The Heap, and Pointers");

    let my_number = 15; // This is an i32;
    let single_reference = &my_number; // This is a &i32
    let double_reference = &single_reference; // This is a &&i32
    let five_reference = &&&&&my_number; // This is a &&&&&i32
    println!(
        "{} is equal to {}? {}",
        double_reference,
        five_reference,
        double_reference == ***five_reference
    );
}

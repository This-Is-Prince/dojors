use std::cmp::PartialOrd;
use std::fmt::Debug; // Debug is located at std::ftm::Debug. So now we can just write 'Debug'.
use std::fmt::Display;

fn return_number_with_type_annotation(number: i32) -> i32 {
    println!("Here is your number.");
    number
}

fn return_number_with_generic_type<T>(number: T) -> T {
    println!("Here is your number.");
    number
}

/*
fn print_number_without_giving_debug_trait<T>(number: T) {
    println!("Here is your number: {:?}", number); // ERROR, `T` doesn't implement `std::fmt::Debug`
}
*/

fn print_number_with_giving_debug_trait<T: Debug>(number: T) {
    // <T: Debug> is the important part
    println!("Here is your number: {:?}", number);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!(
        "{}: Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn compare_and_display_with_where<T, U>(statement: T, num_1: U, num_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{}: Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    // Type T needs Display, Type U needs Display
    println!(
        "I have two things to say: {} and {}",
        statement_1, statement_2
    );
}

#[allow(unused_variables)]
fn main() {
    println!("Generics");

    let number = return_number_with_type_annotation(5);
    println!("return_number_with_type_annotation: {}", number);

    let number = return_number_with_generic_type(8);
    println!("return_number_with_generic_type: {}", number);

    // print_number_without_giving_debug_trait(5);
    print_number_with_giving_debug_trait(5);

    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };

    let number = 55;
    print_item(number);
    print_item(charlie);

    compare_and_display("Listen up!", 9, 8);
    compare_and_display_with_where("Listen up!", 90, 890);

    say_two("Hello there!", String::from("I hate sand.")); // Type T is a &str, but type U is String
    say_two(
        String::from("Where is Abohar?"),
        String::from("Is she all right?"),
    ); // Both types are String.
}

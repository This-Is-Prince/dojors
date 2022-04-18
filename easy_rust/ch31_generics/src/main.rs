use std::cmp::PartialOrd;
use std::fmt::Debug; // Debug is located at std::ftm::Debug. So now we can just write 'Debug'.
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}
/*
fn compare_and_display<T:Display,U:Display+PartialOrd>(statement:T,num_1:U,num_2:U){

} */

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
}

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

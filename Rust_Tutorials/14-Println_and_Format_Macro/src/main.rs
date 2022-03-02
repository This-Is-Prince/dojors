#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    println!("--------Println! and Format! macros--------");
    print!("Partner\n");

    let more_data = 6.7;
    println!("My data is {1} and {0}", 5, more_data);

    println!(
        "My name is {first_name} {last_name}",
        last_name = "Kumar",
        first_name = "Prince"
    );

    #[derive(Debug)]
    struct PrinceData {
        pub a: i32,
        pub b: f32,
    }

    let data = PrinceData { a: 1, b: 1.1 };
    let other_data = PrinceData { a: 2, b: 2.2 };
    println!(
        "Prince data is {1:#?} and other data is {0:#?}",
        data, other_data
    );

    let some_formatted_string = format!(
        "Prince data is {1:#?} and other data is {0:#?}",
        data, other_data
    );

    println!("{}", some_formatted_string);
}

#[allow(unused_variables)]
fn main() {
    println!("Strings");

    let name = "서태지";
    let other_name = String::from("Adrian Fahrenheit Țepeș");

    let name = "😂";
    println!("My name is actually {}", name);

    println!(
        "A String is always {:?} bytes. It is Sized.",
        std::mem::size_of::<String>()
    ); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!(
        "And an i8 is always {:?} bytes. It is Sized.",
        std::mem::size_of::<i8>()
    );
    println!(
        "And an f64 is always {:?} bytes. It is Sized.",
        std::mem::size_of::<f64>()
    );
    println!(
        "But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("서태지")
    ); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!(
        "And Adrian Fahrenheit Țepeș is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("Adrian Fahrenheit Țepeș")
    );

    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );

    // let my_sting = "Try to make this a String".into(); // ERROR, consider giving `my_sting` a type
    let my_string: String = "Try to make this a String".into();
}

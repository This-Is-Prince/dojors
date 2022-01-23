#[allow(unused)]
fn main() {
    let name = "서태지";
    let other_name = String::from("서태지");

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
        "And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("Adrian Fahrenheit Țepeș")
    );
    println!(
        "And String::from('Adrian Fahrenheit Țepeș') is always {:?} bytes. It is Sized.",
        std::mem::size_of_val(&String::from("Adrian Fahrenheit Țepeș"))
    );
    println!(
        "And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.",
        std::mem::size_of_val("Adrian Fahrenheit")
    );
    println!(
        "And String::from('Adrian Fahrenheit') is always {:?} bytes. It is Sized.",
        std::mem::size_of_val(&String::from("Adrian Fahrenheit"))
    );

    let name: String = String::from("Prince");
    let name: String = "Prince".to_string();
    let name: String = format!("{}", "Prince");
    let name: String = "Prince".into(); // Type annotations needed for into

    let my_name = "Billy Roy";
    let my_country = "USA";
    let my_home = "Korea";
    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );
}

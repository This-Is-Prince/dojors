#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    println!("-----Rust Tuples-----");

    let some_tuple = (2, 3.4);
    println!("My data is {} {}", some_tuple.0, some_tuple.1);
    println!("My full tuple is {:?}", some_tuple);

    let some_tuple = (2, 3.4, "a", "b".to_string(), 'c', (1.2, 2.3));
    let some_val = (some_tuple.5).1;
    println!("value of some_val is {}", some_val);

    let some_color = get_some_rgb();
    println!("Green is {}", some_color.1);

    let (my_red, my_green, my_blue) = some_color;
    println!("r {} g {} b {}", my_red, my_green, my_blue);

    let empty_tuple = ();
    match some_color.2 {
        0..=200 => println!("0 to 200"),
        _ => (),
    }
}

#[allow(dead_code)]
fn some_procedure() {
    ()
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_some_rgb() -> (u8, u8, u8) {
    // Some Logic...
    (200, 201, 202)
}

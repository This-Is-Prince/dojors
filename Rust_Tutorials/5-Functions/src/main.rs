mod main1;

#[allow(unused_variables)]
fn main() {
    /* println!("Functions and Procedures");
    let returned_data = some_function(2.2, 50);
    println!("returned_data is {}", returned_data);

    some_procedure(2.3, 1);

    some_str_procedure("test");

    let string_slice_var: &str = "Howdy!";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a REAL String :)");
    // some_str_procedure(string_var); // Error
    some_str_procedure(&string_var);

    some_string_procedure(string_var);
    // some_string_procedure(string_var); // Error
    // some_string_procedure(&string_var); // Error

    let string_var1 = String::from("I'm a REAL String 1 :)");
    some_string_procedure1(&string_var1);
    some_string_procedure1(&string_var1);
    some_string_procedure1(&string_var1);
    some_string_procedure1(&string_var1);
    some_string_procedure1(&string_var1); */

    main1::main1();
}
/*
fn some_string_procedure1(param: &String) {
    println!("I'm in some_string_procedure with param {}", param);
}

fn some_string_procedure(param: String) {
    println!("I'm in some_string_procedure with param {}", param);
}

fn some_str_procedure(param: &str) {
    println!("I'm in some_str_procedure with param {}", param);
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

// #[allow(dead_code)]
fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I'm in some_function!");

    // 10.1 * param_a // No semicolon means this is what's returned by the function
    let return_var = 10. * param_a + param_b as f32;
    return_var // No semicolon means this is what's returned by the function
}
 */

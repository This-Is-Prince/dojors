#[allow(unused_variables)]

fn main() {
    println!("-----Rust Functions and Procedures-----");

    let returned_data = some_function(2.2, 50);
    println!("returned_data is {}", returned_data);

    some_procedure(2.3, 1);

    some_str_procedure("test");

    let string_slice_var: &str = "Prince";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a REAL String :)");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);

    // some_string_procedure(string_var);
    // some_string_procedure(string_var); // Error string_var is not available it is moved

    some_string_procedure2(&string_var);
    some_string_procedure2(&string_var);
    some_string_procedure2(&string_var);
    some_string_procedure2(&string_var);
}

#[allow(dead_code)]
fn some_string_procedure2(param: &String) {
    println!("I'm in some_string_procedure with param {}", param);
}
#[allow(dead_code)]
fn some_string_procedure(param: String) {
    println!("I'm in some_string_procedure with param {}", param);
}

#[allow(dead_code)]
fn some_str_procedure(param: &str) {
    println!("I'm in some_str_procedure with param {}", param);
}

#[allow(dead_code)]
fn some_procedure(param_a: f32, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I'm in some_function!");
    // 3.23 // No semicolon means this is what's returned by the function
    let return_var = 10. * param_a + param_b as f32;
    return_var
}

#[allow(unused_variables)]
fn main() {
    let returned_data = some_function(5.1, 50);
    println!("returned_data is {}", returned_data);
    // Hello();
    some_procedure(2.3, 1);
    some_str_procedure("test");

    let string_slice_var = "Howdy!";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a Real String :)");
    some_str_procedure(&string_var);
    some_string_procedure(string_var);
}

fn some_string_procedure(param: String) {
    println!("I'm in some_str_procedure with param {}", param)
}
fn some_str_procedure(param: &str) {
    println!("I'm in some_str_procedure with param {}", param)
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

#[allow(dead_code)]
fn some_function(param_a: f32, _param_b: i128) -> f32 {
    println!("I'm in some_function!");
    // 10.1 // No semicolon means this is what's returned by the function
    // 10f32
    // 10 as f32
    // 10.

    // let return_var = 10.1 * param_a + _param_b as f32;
    // return_var

    if param_a < 100. {
        let return_var = 10.1 * param_a + _param_b as f32;
        return_var
    } else {
        -1.
    }
}

// #[allow(non_snake_case)]
// fn Hello() {
//     println!("hello")
// }

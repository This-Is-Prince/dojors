#[allow(unused_variables)]

fn main() {
    // some_function(2.1, 2);

    some_procedure(1.1, 1);
    some_str_procedure("test");

    let string_slice_var: &str = "Howdy!";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a REAL String :)");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var);

    // some_string_procedure(string_var);
    // some_string_procedure(string_var); // Error

    // OR
    some_string_procedure2(&string_var);
    some_string_procedure2(&string_var);
}

fn some_string_procedure2(param: &String) {
    println!("I'm in some_string_procedure with param {}", param);
}

#[allow(dead_code)]
fn some_string_procedure(param: String) {
    println!("I'm in some_string_procedure with param {}", param);
}

fn some_str_procedure(param: &str) {
    println!("I'm in some_str_procedure with param {}", param);
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

#[allow(dead_code)]
fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("some_function");
    // 10.1f32
    // 10.1
    // 10.1 as f32

    if param_a < 100. {
        let return_ar = 10.1 * param_a + param_b as f32;
        // return return_ar;
        return_ar
    } else {
        -1.
    }
}

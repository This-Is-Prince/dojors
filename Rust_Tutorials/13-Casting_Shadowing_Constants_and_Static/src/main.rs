#[allow(unused_variables)]
fn main() {
    println!("--------Casting, Shadowing, Constants and Static--------");

    // Casing
    // casting();

    // Shadowing
    // shadowing();

    // Constants
    // constants();

    // Static
    static_variables();
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn casting() {
    let some_i32: i32 = 10;
    let some_f64: f64 = 20.2;

    let combined = some_i32 + some_f64 as i32;
    println!("{}", combined);
    let combined = some_i32 as f64 + some_f64;
    println!("{}", combined);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn shadowing() {
    let var_a: i32 = 10;

    {
        println!("The Inner scope can see the outer scope var_a of {}", var_a);
        let var_a: f32 = 20.222; // shadowing
        println!("But it can 'Shadow' it with it's own version of {}", var_a);
    }
    println!("See, var_a for the outer scope is still {}", var_a);
}

const PRINCE_CONSTANT: i64 = 100;
#[allow(unused_variables)]
#[allow(dead_code)]
fn constants() {
    println!("Prince constant is {}", PRINCE_CONSTANT);

    let circle_pi = std::f32::consts::PI;
    println!("{}", circle_pi);
}

static mut MY_STATIC_VARIABLE: i32 = 10;
#[allow(unused_variables)]
#[allow(dead_code)]
fn static_variables() {
    // MY_STATIC_VARIABLE = 20; // use of mutable static is unsafe and requires unsafe function or block
    // println!("{}", MY_STATIC_VARIABLE); // ERROR, use of mutable static is unsafe and requires unsafe function or block

    unsafe {
        MY_STATIC_VARIABLE = 20;

        println!("{}", MY_STATIC_VARIABLE);
    }
}

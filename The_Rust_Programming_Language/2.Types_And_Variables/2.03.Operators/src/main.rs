/**
 * Arithmetic Operator Function
 */
// fn arithmetic() {
//     let mut a = 2 + 3 * 4; // +-*/
//     println!("a = {}", a);
//     a = a + 1; // rust does't support --, ++
//     a -= 2; // a=a-2;
//     a += 2; // a=a+2;
//     println!("a = {}", a);
//     println!("remainder of {} / {} = {}", a, 3, (a % 3));
//     let a_cubed = i32::pow(a, 3);
//     println!("a_cubed = {}", a_cubed);
//     let b = 2.5;
//     let b_cubed = f64::powi(b, 3);
//     println!("b_cubed = {}", b_cubed);
//     let b_to_pi = f64::powf(b, std::f64::consts::PI);
//     println!("b_to_pi = {}", b_to_pi);
// }

/**
 * Bitwise Operator Function
 */
fn bitwise() {
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("c = {}", c); // c = 3
                           // shift operator
    let two_to_10 = 1 << 10; // 1 in binary = 01, << operator shift 1 by 10 places 10_000_000_000 now this number is 1024 in decimal
    println!("2^10 = {}", two_to_10); // 2^10 = 1024

    // logical operator
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("pi_less_4 = {}", pi_less_4);
    // >, <=, >=, ==
    let x = 5;
    let x_is_5 = x == 5;
    println!("x_is_5 = {}", x_is_5);
}

/**
 * Operator Function
 */
fn operator() {
    // Arithmetic
    // arithmetic();

    // bitwise operator
    bitwise();
}

/**
 * Main Function
 */
fn main() {
    operator();
}

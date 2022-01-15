// ----------3rd-----------
fn main() {
    let some_int_var = 10;
    let result_ref = get_int_ref(&some_int_var);
    println!("{}", result_ref);
}

fn get_int_ref<'a>(param_1: &'a i32) -> &'a i32 {
    param_1
}

// fn get_int_ref<'a>(param_1: &i32) -> &i32 {
//     param_1
// }

// fn get_int_ref(param_1: &i32) -> &i32 {
//     param_1
// }

// fn get_int_ref() -> i32 {
//     let a = 1;
//     a
// }

// fn get_int_ref() -> &i32 {
//     let a = 1;
//     &a
// }

// ----------2nd-----------
// fn main() {
//     let a;
//     {
//         let b = String::from("Howdy");
//         a = &b;
//     }
//     println!("a is {}", a);
// }

// ----------1st-----------
// fn main() {
//     let a;
//     {
//         let b = String::from("Howdy");
//         a = b;
//     }
//     println!("a is {}", a);
// }

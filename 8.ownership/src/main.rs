// fn main() {
//     let hello_world = "Hello World!";
//     let copy_hello_world = hello_world;
//     println!(
//         "The value of hello_world and copy_hello_world is : {},{}",
//         hello_world, copy_hello_world
//     );
// }

// fn main() {
//     let hello_world = String::from("Hello World!");
//     // let copy_hello_world = hello_world; //hello_world data moves to copy_hello_world (not copied);
//     let copy_hello_world = hello_world.clone();
//     println!(
//         "The value of hello_world and copy_hello_world is : {},{}",
//         hello_world, copy_hello_world
//     );
// }

// fn main() {
//     let mut string = String::from("Hello");
//     string.push_str(" World!");
//     println!("The value of string is : {}", string);
// }

// fn main() {
//     let a = 6;
//     let x = a;
//     println!("The value of a and x is : {},{}", a, x);
// }

// fn main() {
//     let a = 6;
//     print_a(a);
//     println!("The value of a after print is : {}", a);
// }
// fn print_a(a: i32) {
//     println!("The value of a is : {}", a);
// }

// fn main() {
//     let hello_world = String::from("Hello World!");
//     print_hello_world(hello_world);
//     // println!("The value of hello_world after print is : {}", hello_world);//Error
// }
// fn print_hello_world(hello_world: String) {
//     println!("The value of hello_world is : {}", hello_world);
// }

// fn main() {
//     let hello_world = "Hello World!";

//     print_hello_world(hello_world);
// }
// fn print_hello_world(hello_world: &str) {
//     println!("The value of hello_world is : {}", hello_world);
// }

// fn main() {
//     let mut hello = String::from("Hello");
//     hello = print_hello(hello);
//     println!("THe value of hello after print is : {}", hello);
// }
// fn print_hello(hello: String) -> String {
//     println!("The value of hello is : {}", hello);
//     hello
// }
// fn main() {
//     let hello = String::from("Hello");
//     let (hello, len) = print_hello(hello);
//     println!("THe value of hello after print is : {}", hello);
//     println!("THe len of hello is : {}", len);
// }
// fn print_hello(hello: String) -> (String, usize) {
//     println!("The value of hello is : {}", hello);
//     let len = hello.len();
//     (hello, len)
// }

// ==========
// Borrowing
// ==========

// fn main() {
//     let hello = String::from("Hello");
//     let len = calculate_len(&hello);
//     println!("The length of string {} is {}", hello, len);
// }
// fn calculate_len(hello: &String) -> usize {
//     hello.len()
// }

// fn main() {
//     let mut hello = String::from("Hello");
//     change(&mut hello);
//     println!("The value of hello is : {}", hello);
// }
// fn change(hello: &mut String) {
//     hello.push_str("World!");
// }

// fn main() {
// let mut s = String::from("Hello");
// let a = &mut s;
// let b = &s;//Error
// let b = &mut s;//Error
// println!("The value of a and b is : {},{}", a, b);
// }

// fn main() {
//     let mut s = String::from("HEllo");
//     // let r1=&mut s;//Error
//     {
//         let r2 = &mut s;
//         println!("THe value of r2 is : {}", r2);
//         r2.push_str("World");
//         println!("The value of r2 after change is : {}", r2);
//     }
//     let r1 = &mut s;
//     println!("The value of r1 is : {}", r1);
// }

fn main() {
    let mut s = String::from("Hello");
    // let r1=&mut s;//Error
    change(&mut s);
    let r1 = &mut s;
    println!("The value of r1 is : {}", r1);
}
fn change(string: &mut String) {
    string.push_str(" Bro");
}

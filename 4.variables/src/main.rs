// Error
// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// const CONST: u32 = 90;
// // let CONST=90;//Error
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
//     println!("The value of CONST is : {}", CONST);
// }

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
    let mut x = "Print";
    println!("The value of x is: {}", x);
    x = "Hello Bro";
    println!("The value of x is: {}", x);
}

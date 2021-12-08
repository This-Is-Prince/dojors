// use std::io;

// fn main() {
//     // Integer
//     // Unsigned
//     // let a: u8 = 255; //0 - 255 limit
//     // let b: u16 = 65535; //0 - 65535 limit
//     // let c: u32 = 600000000;
//     // let d: u64 = 9999999999999999999;
//     // println!("The value of a,b,c,d is {},{},{},{}", a, b, c, d);
//     // signed
//     // let a: i8 = 127; //-128 - 127
//     // let b: i16 = -32768; //-32768 - 32767
//     // let c: i32 = 12732243;
//     // let d: i64 = 1275555;
//     // println!("The value of a,b,c,d is {},{},{},{}", a, b, c, d);

//     // Float
//     // let a: f32 = 45.4;
//     // let b: f64 = 4552.2222222222233333333333333333333333333333333333;
//     // println!("The value of a,b is {},{}", a, b);

//     // Boolean
//     // let isFree = false;
//     // let isOpen: bool = true;
//     // println!("The value of isFree,isOpen is {},{}", isFree, isOpen);

//     // Character
//     // let ch = 'a';
//     // println!("The value of ch is {}", ch);

//     // Compound Types
//     // let tup: (char, bool, i32, f32) = ('a', false, 77, 44.5225);
//     // println!("The value of tup.0 is : {}", tup.0);
//     // println!("The value of tup.1 is : {}", tup.1);
//     // println!("The value of tup.2 is : {}", tup.2);
//     // println!("The value of tup.3 is : {}", tup.3);

//     // let tup = ('b', false, 55, 54.34);
//     // println!("The value of tup.0 is : {}", tup.0);
//     // println!("The value of tup.1 is : {}", tup.1);
//     // println!("The value of tup.2 is : {}", tup.2);
//     // println!("The value of tup.3 is : {}", tup.3);

//     // let (a, b, c, d) = ('b', false, 55, 54.34);
//     // println!("The value of tup.0 is : {}", a);
//     // println!("The value of tup.1 is : {}", b);
//     // println!("The value of tup.2 is : {}", c);
//     // println!("The value of tup.3 is : {}", d);

//     // Unit type
//     // let unit: (u32) = (3);
//     // println!("The value of unit is: {}", unit);

//     // Array type
//     // let arr = [1, 2, 3, 4, 5];
//     // let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // let arr = [3, 4];
//     // let arr = [3; 4];
//     // println!("arr[0] : {}", arr[0]);
//     // println!("arr[1] : {}", arr[1]);
//     // println!("length : {}", arr.len());

//     // Example
//     let a = [1, 2, 3, 4, 5];
//     println!("Please enter an array index.");
//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
//     let element = a[index];
//     println!(
//         "The value of the element at index {} is: {}",
//         index, element
//     );
// }

// fn main() {
//     let x: i32 = return_integer();
//     println!("The value of  x is : {}", x);

//     // let x = [3; 5];
//     // let x: [f64; 5] = [1.2, 3.2, 3.2, 4.2, 5.5];
//     // let x: [f64; 5] = [1.2; 5];
//     // for element in x {
//     //     println!("The value of element is : {}", element);
//     // }

//     // for element in 1..5 {
//     //     println!("The value of element is : {}", element);
//     // }
//     // for element in (1..5).rev() {
//     //     println!("The value of element is : {}", element);
//     // }
//     let (x, y, z) = point();
//     println!("The value of x point is : {}", x);
//     println!("The value of y point is : {}", y);
//     println!("The value of z point is : {}", z);
// }
// fn return_integer() -> i32 {
//     5
// }
// fn point() -> (i32, i32, i32) {
//     return (1, 2, 3);
// }

fn main() {
    // let tup: (u8, u16, u128) = (4, 5, 65);
    // println!("The value of tup.0 is {}", tup.0);
    // println!("The value of tup.1 is {}", tup.1);
    // println!("The value of tup.2 is {}", tup.2);
    // let (x, y, z) = (4, 5, 6);
    // println!("The value of x is {}", x);
    // println!("The value of y is {}", y);
    // println!("The value of z is {}", z);

    let arr = [1, 2, 3, 4];
    let arr2 = [3; 5];
    let arr3: [u8; 5];
    arr3 = [1, 2, 3, 4, 5];
}

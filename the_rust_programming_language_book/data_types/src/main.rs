use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess is: {}", guess);

    let x = 2.0;
    let y: f32 = 3.0;
    println!("x={x}, y={y}");

    let t = true;
    let f: bool = false;
    println!("t={}, f={}", t, f);
    // let x: u32;
    // println!("{x}")

    let x = 'x';
    let z: char = 'Z';
    let heart_eyed_smile = '😍';
    println!("x={}, z={}, heart_eyed_smile={}", x, z, heart_eyed_smile);


    let tup:(i32,f64, u8, char) = (-500, 6.4, 200, 'a');
    println!("tup: ({}, {}, {}, {})", tup.0, tup.1, tup.2, tup.3);
    let (a,b,c,d) = tup;
    println!("tup: ({}, {}, {}, {})", a, b, c, d);

    let unit_tup=();
    println!("unit_tup: {:?}", unit_tup);

    let arr =[1,2,3,4,5];
    println!("arr: {:?}", arr);
    println!("arr[0]: {}", arr[0]);

    let arr1: [i32;5] = [-1,-2,1,2,0];
    println!("arr1: {:?}", arr1);

    let arr2 = [4;5];
    println!("arr2: {:?}", arr2);

    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please type a number!");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}

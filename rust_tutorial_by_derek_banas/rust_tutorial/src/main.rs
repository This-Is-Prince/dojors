#![allow(unused)]

use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

use std::collections::HashMap;
use std::ops::Add;

fn main() {
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";

    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't Receive Input");

    // println!("Hello, {}! {}", name.trim_end(), greeting);

    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age = "47";
    // let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f64 : {}", f64::MAX);

    // let is_true: bool = true;
    // let my_grade: char = 'A';

    // let num_1: f32 = 1.11111_11111_11111;
    // println!("f32 : {}", num_1 + 0.11111_11111_11111);

    // let num_2: f64 = 1.11111_11111_11111;
    // println!("f64 : {}", num_2 + 0.11111_11111_11111);

    // let age = rand::thread_rng().gen_range(1..101);
    // println!("Age : {}", age);

    // if (age >= 1) && (age <= 18) {
    //     println!("Important Birthday");
    // } else if (age >= 21) && (age <= 50) {
    //     println!("Other")
    // }

    // let can_vote = if age >= 18 { true } else { false };
    // println!("Can vote : {}", can_vote)

    // match age {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("Most Important Birthday"),
    //     65..=100 => println!("Parents Birthday"),
    //     _ => println!("Don't know who's birthday it is."),
    // }

    // let voting_age = 18;

    // match age.cmp(&voting_age){
    //     Ordering::Less=>println!("Can't Vote"),
    //     Ordering::Greater=>println!("Can Vote"),
    //     Ordering::Equal=>println!("You gained the right to vote"),
    // }

    // let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // println!("1st : {}", arr_1[0]);
    // println!("Length : {}", arr_1.len());

    // let mut loop_idx = 0;
    // loop {
    //     if arr_1[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_1[loop_idx] == 9 {
    //         break;
    //     }
    //     println!("Val : {}", arr_1[loop_idx]);
    //     loop_idx += 1;
    // }

    // while loop_idx < arr_1.len() {
    //     println!("Arr : {}", arr_1[loop_idx]);
    //     loop_idx += 1;
    // }

    // for val in arr_1.iter() {
    //     println!("Val : {}", val);
    // }

    // let my_tuple: (u8, String, f64) = (47, "Prince".to_string(), 50_000.00);
    // println!("Name : {}", my_tuple.1);

    // let (v1, v2, v3) = my_tuple;
    // println!("Age : {}", v1)

    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str(" word");

    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }

    // let st2 = st1.replace("A", "Another");
    // println!("{}, {}", st1, st2);

    // let st3 = String::from("x r t b h k k a m c ");
    // let mut v1: Vec<_> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();

    // for c in v1 {
    //     println!("{}", c);
    // }

    // let st4 = "Random String";
    // let mut st5 = st4.to_string();
    // println!("{}", st5);
    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..6];
    // println!("String length : {}", st6.len());
    // st5.clear();
    // println!("St5 : {}", st5);
    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7;

    // for c in st8.bytes() {
    //     println!("{}", c);
    // }

    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // enum Day {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday,
    // }

    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Day::Saturday | Day::Sunday => true,
    //             _ => false,
    //         }
    //     }
    // }
    // let today: Day = Day::Monday;
    // match today {
    //     Day::Monday => println!("Everyone hates Monday"),
    //     Day::Tuesday => println!("Donut day"),
    //     Day::Wednesday => println!("Hump day"),
    //     Day::Thursday => println!("Pay day"),
    //     Day::Friday => println!("Almost Weekend"),
    //     Day::Saturday => println!("Weekend"),
    //     Day::Sunday => println!("Weekend"),
    // }
    // println!("Is today the weekend {}", today.is_weekend());

    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2 = vec![1, 2, 3, 4];
    // vec2.push(5);
    // println!("1st : {}", vec2[0]);
    // let second = &vec2[1];

    // match vec2.get(1) {
    //     Some(second) => println!("{}", second),
    //     None => println!("None"),
    // }

    // for i in &mut vec2 {
    //     *i *= 2;
    // }
    // for i in &vec2 {
    //     println!("{}", i);
    // }
    // println!("Vec Length {}", vec2.len());
    // println!("Pop : {:?}", vec2.pop());

    // say_hello();
    // print_sum(4, 6);

    // println!("{} + {} = {}", 4, 7, get_sum(4, 7));
    // println!("{:?}", get_two_val(64));

    // let num_list = vec![1, 2, 3, 4, 5];
    // println!("Sum of list = {}", sum_list(&num_list));

    // println!("5 + 4 = {}", get_sum_gen(5, 4));
    // println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));

    // let str1 = String::from("World");
    // let str2 = str1;
    // println!("Hello {}", str1); // Error, borrow of moved value: `str1`

    // let str2 = str1.clone();
    // println!("Hello {}", str1);
    // print_str(str1);
    // let str3 = print_return_str(str1);
    // println!("str3 = {}", str3);
    // let mut str1 = String::from("Prince");
    // change_string(&mut str1);

    // let mut heroes = HashMap::new();
    // heroes.insert("Superman", "Clark Kent");
    // heroes.insert("Batman", "Bruce Wayne");
    // heroes.insert("Flash", "Barry Allen");
    // // println!("{:?}", heroes);

    // for (k, v) in heroes.iter() {
    //     println!("{} = {}", k, v);
    // }

    // println!("Length : {}", heroes.len());

    // if heroes.contains_key("Batman") {
    //     let the_batman = heroes.get("Batman");
    //     match the_batman {
    //         Some(x) => println!("Batman is a hero"),
    //         None => println!("Batman is not a hero"),
    //     }
    // }

    // struct Customer {
    //     name: String,
    //     address: String,
    //     balance: f32,
    // }
    // let mut bob = Customer {
    //     name: String::from("Bob Smith"),
    //     address: String::from("555 Main St"),
    //     balance: 234.50,
    // };
    // bob.address = String::from("505 Main St");

    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U,
    // }

    // let rec = Rectangle {
    //     length: 4,
    //     height: 10.5,
    // };

    const PI: f32 = 3.14;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            Self { length, width }
        }
        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            Self { length, width }
        }
        fn area(&self) -> f32 {
            (self.length / 2.0).powf(2.0) * PI
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let cir: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area : {}", rec.area());
    println!("Cir Area : {}", cir.area());
}

// fn print_str(x: String) {
//     println!("A string {}", x);
// }

// fn print_return_str(x: String) -> String {
//     println!("A string {}", x);
//     x
// }

// fn change_string(name: &mut String) {
//     name.push_str(" is happy");
//     println!("Message : {}", name);
// }

// fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// fn say_hello() {
//     println!("Hello");
// }

// fn print_sum(x: i32, y: i32) {
//     println!("{} + {} = {}", x, y, x + y);
// }

// fn get_sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn get_two_val(x: i32) -> (i32, i32) {
//     (x + 1, x + 2)
// }

// fn sum_list(list: &[i32]) -> i32 {
//     let mut sum = 0;
//     for &ele in list.iter() {
//         sum += ele;
//     }
//     sum
// }

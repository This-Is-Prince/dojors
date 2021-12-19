fn main() {
    /*
    ----- Ownership Rules -----
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    // {
    //     // s is not valid here, it's not yet declared
    //     let s = "hello"; // s is valid from this point forward
    //                      // do stuff with s
    // } // this scope is now over, and s is no longer valid

    // {
    //     // s is not valid here, it's not yet declared
    //     let s: String = String::from("hello"); // s is valid from this point forward
    //                                            // do stuff with s
    // } // this scope is now over, and s is no longer valid

    /*
    Memory and allocation
    */
    // let x = 5;
    // let y = x; // Copy

    // let s1 = String::from("Hello");
    // let s2 = s1; // Move (not shallow copy or deep copy)

    // println!("{}, world!", s1); // Error

    // let s1 = String::from("Hello");
    // let s2 = s1.clone(); // Cloning

    // println!("s1 - > {}, world!", s1);
    // println!("s2 - > {}, world!", s2);

    /*
    Ownership and Functions
    */
    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s); //Error s is not value because s move in takes_ownership function

    // let x = 5;
    // makes_copy(x);
    // println!("{}", x);

    // let s1 = gives_ownership();
    // let s2 = String::from("hello2");
    // let s3 = takes_and_gives_back(s2);
    // println!("s1 = {} , s3 = {}", s1, s3);

    /*
    References and Borrowing
    */
    // let s1 = String::from("Hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // let s1 = String::from("Hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // let mut s1 = String::from("hello");
    // change(&mut s1);
    // println!("{}", s1);

    // let mut st = String::from("HI");
    // let r1 = &mut st;
    // let r2 = &mut st;
    // reference(r1);
    // reference(r2);

    // fn reference(st: &mut String) {
    //     println!("{} ", st);
    // }

    /*
    The Slice Type
     */
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    let mut s = String::from("hello world");
    // let hello=&s[0..5];
    let hello = &s[..5];
    // let world=&s[6..11];
    let world = &s[6..];
    let hello_world = &s[..];
}
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }
// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); //len() returns the length of a string
//     (s, length)
// }
// fn calculate_length(s: &String) -> usize {
// // s.push_str("oops");// references are immutable by default.
//     let length = s.len(); //len() returns the length of a string
//     length
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world")
// }

/*
The Rule of Reference
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
*/

/*
The Slice Type
 */

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

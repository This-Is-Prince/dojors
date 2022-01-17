#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    // ----------Ownership Rules----------
    // 1. Each value in Rust has variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    /*     {
        // s is not valid here, it's not yet declared
        let _s: &str = "hello"; // s is valid from this point forward
                                // do stuff with s
    } //this scope is now over, and s is no longer valid
    */

    /*
    let x: i32 = 5;
    let y: i32 = x; // Copy
    */

    /*
    let s1: String = String::from("hello");
    let s2: String = s1; // s1 move to s2 now s1 is no longer valid
    // println!("{}, world!", s1); // error
    */

    /*
    let s1: String = String::from("hello");
    let s2: String = s1.clone();
    println!("{}, world!", s1);
    */

    /*
    let s: String = String::from("hello");
    takes_ownership(s); // s move to takes_ownership function, now s no longer valid
    println!("{}", s); // Error

    let x: i32 = 5;
    makes_copy(x); // Copy
    println!("{}", x);
    */

    /*
    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
     */

    /*
       let s1: String = String::from("hello");
       let (s2, len): (String, usize) = calculate_length(s1);
       println!("The length of '{}' is {}.", s2, len);
    */

    /*
    let s1: String = String::from("hello");
    let len: usize = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    */

    /*
    let mut s1: String = String::from("hello");
    change(&mut s1);
     */

    /*
    let mut s: String = String::from("hello");
    let r1: &mut String = &mut s;
    let r2: &mut String = &mut s; // Error
    println!("{}, {}", r1, r2);
     */

    /*
    let mut s: String = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;
    println!("{}, {}", r1, r2);
     */

    /*
    let mut s: String = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;
    let r3: &mut String = &mut s; // Error
    println!("{}, {}", r1, r2);
     */

    /*
    let mut s: String = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;
    println!("{}, {}", r1, r2);
    let r3: &mut String = &mut s;
    println!("{}", r3);
    */

    /*
    let reference_to_nothing: &String = dangle();
     */

    //  The Rules of Reference
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.

    // Slices

    /*
       let mut s: String = String::from("hello world");
       let word: usize = first_word(&s);
       s.clear();
       println!("{}", word);
    */

    /*
    let mut s: String = String::from("hello world");
    let hello: &str = &s[0..5]; // &s[..5] same
    let world: &str = &s[6..11]; // &s[6..] same
    let hello_world: &str = &s[..];

    let word = first_word(&s);
    s.clear(); // Error because we are using word after this clear word refer to s string part
    println!("word is: {}", word);
     */

    /*
    let mut s: String = String::from("hello world");
    let s2: &str = "hello world";
    let word: &str = first_word(&s);
    let word: &str = first_word(s2);
    let word: &str = first_word(&s2);
     */

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[0..2];
}

/*
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
 */

/*
fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
 */
/*
fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
 */
/*
fn dangle() -> &String // Error
{
    let s: String = String::from("hello");
    &s
}
 */

/*
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
 */

/*
fn calculate_length(s: &String) -> usize {
    let length: usize = s.len();
    length
}
 */
/*
fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len(); // len() returns the length of a String
    (s, length)
}
 */

/*
fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/

/*
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
 */

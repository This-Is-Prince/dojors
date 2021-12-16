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

    let s1 = gives_ownership();
    let s2 = String::from("hello2");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {} , s3 = {}", s1, s3);
}
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

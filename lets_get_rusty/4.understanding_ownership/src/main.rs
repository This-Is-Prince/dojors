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

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // Cloning

    println!("s1 - > {}, world!", s1);
    println!("s2 - > {}, world!", s2);
}

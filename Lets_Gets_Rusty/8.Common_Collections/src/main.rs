#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use unicode_segmentation::UnicodeSegmentation;

#[allow(unused_variables)]
fn main() {
    /* ============Vector============ */
    /*
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];
    */

    /*
       let v = vec![1, 2, 3, 4, 5];
       let third = &v[2]; // Careful index out of bounds error can occur if index greater than or equal to vec len
       println!("The third element is {}", third);
    */

    // OR

    /*
    let v = vec![1, 2, 3, 4, 5];
    let index: usize = 3;
    match v.get(index) {
        Some(third_elm) => {
            println!("The third element is {}", third_elm);
        }
        None => println!("No Element at index {}", index),
    }
    */

    /*
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    let index: usize = 3;
    v.push(6); // Error cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("The third element is {}", third);
    match v.get(index) {
        Some(third_elm) => {
            println!("The third element is {}", third_elm);
        }
        None => println!("No Element at index {}", index),
    }
     */

    /* ============Iteration over Vector============ */
    /*
    let mut v = vec![1, 2, 3, 4, 5];
    /*
    for elm in v {
        println!("{}", elm);
    }
    */

    // OR getting references
    for elm in &mut v {
        *elm *= 50;
    }
    println!("vector v is: {:#?}", v);
    */

    /* ============Storing enum variants inside of a  Vector============ */
    /*
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.2),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        other => println!("Not a integer!, other thing {:#?}", other),
    }
    */

    /* ============String============ */
    // Strings are stored as a collection of UTF-8 encoded bytes
    /*
    let s1: String = String::new();
    let s2: &str = "initial contents";
    let s3: String = s2.to_string();
    let s4: String = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("World!");
    */

    /*
    let s3: String = s1 + &s2; // s1 move here, s1 no longer valid
    println!("{}", s1); // Error
    */

    /*     let s3: String = format!("{}{}", s1, s2);
    println!("{}", s1); // s1 valid here because format! macro does not take ownership of s1 and s2 strings
     */

    /*
    let hello: String = String::from("Hello");
    let i: usize = 0;
    let c: char = hello[i]; // Error cannot access string character using indexing
     */

    /*
    let hello: String = String::from("नमस्ते");
    //  Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for b in hello.as_bytes() {
        println!("{} is ASCII code of '{}' Character", b, *b as char);
    }

    // Scalar values
    // ['न', 'म', 'स', '्', 'त', 'े']
    for c in hello.chars() {
        println!("{}", c);
    }

    // Grapheme clusters
    // ["न", "म", "स्", "ते"]
    for g in hello.graphemes(true) {
        println!("{}", g);
    }
     */

    /* ============HashMap============ */
    /*
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10); // blue move to insert func
    scores.insert(yellow, 50); // yellow move to insert func

    // println!("{}",blue); // Error because blue is not exist
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    if let Some(score) = score {
        println!("{}", score);
    }

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }
    */

    /*
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);
    println!("{:#?}", scores);
     */

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        /*
        let count = map.entry(word).or_insert(0);
        *count += 1;
        */

        *map.entry(word).or_insert(0) += 1;
    }
    println!("{:#?}", map);
}

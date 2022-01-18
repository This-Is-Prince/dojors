use std::fmt::Display;

#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(unused_variables)]
fn main() {
    /* =============10============ */

    /* =============9============ */
    let s: &'static str = "I have a static lifetime.";

    /* =============7============ */
    let novel = String::from("Call me ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /* =============1============ */
    /*
    let r: &i32;
    {
        let x: i32 = 5;
        r = &x; // Error x does not live long enough
    }
    println!("r: {}", r);
    */

    /* =============2============ */
    /*
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    */

    /* =============3============ */
    /*
    let string1 = String::from("abcd");
     let string2 = String::from("xyz");

     let result = longest(string1.as_str(), string2.as_str());
     println!("The longest string is {}", result);
     */

    /* =============4============ */
    /*
    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    */

    /* =============5============ */
    /*
    let string1 = String::from("abcd");
     let result: &str;
     {
         let string2 = String::from("xyz");
         result = longest(string1.as_str(), string2.as_str()); // Error
     }
     println!("The longest string is {}", result);
    */

    /* =============6============ */
    let string1 = String::from("abcd");
    let result: &str;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    /* =============8============ */
}

/*
&i32         // a reference
&'a i32      // a reference with an explicit lifetime
&'a mut i32  // a mutable reference with an explicit lifetime
*/

/*
  1. Each parameter that is a reference gets its own lifetime parameter
  2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters;
  3. If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters.
*/
#[allow(unused_variables)]
#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[allow(unused_variables)]
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
 */

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let example_str: &str = "Howdy";
    let example_string: String = String::from("Partner");

    // translate
    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_hardcoded: String = String::from("Some hardcoded");
    let string_from_str_var: String = String::from(example_str);

    let str_from_string: &str = &example_string;

    // combined strings
    // let test="first"+"second"; // Error
    let combine_string_literals: String = ["first", " second"].concat();
    let combine_with_format_macro: String = format!("{} {}", "first", "second");

    // let string_plus_str = example_str + example_string; // Error
    let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcoded literal");
    let hi = String::from("HI There!");
    // mut_string.push_str(hi); // Error
    mut_string.push_str(&hi);
    mut_string.push('m');
    // mut_string.push("m"); // Error require single quote

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;

    let str_from_substring: &str = &example_str[0..2]; // 2 is not included
    let str_from_substring: &str = &hi[0..=1]; // &hi[..2] same
    let str_from_substring: &str = &hi[..]; // &hi[0..hi.len()] same
                                            // let str_from_string: &str = &example_str[0..200]; // This will ERROR

    let char_by_index = example_str.chars().nth(1);
    if let Some(nth_char) = char_by_index {
        println!("nth char is: {}", nth_char);
    }
}

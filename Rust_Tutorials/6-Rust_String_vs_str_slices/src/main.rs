#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // string slices
    let example_str: &str = "Howdy";
    // string
    let example_string: String = String::from("Partner");

    let string_from_str: String = example_str.to_string();
    // let string_from_str2: String = "Some hardcoded string"; // error
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_string;

    // let test = "First" + " second"; // Error
    let combine_string_literals = ["First", " ", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "First", "second");
    println!("{}", combine_with_format_macro);

    // let string_plus_str = example_str + example_string ; //Error
    let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcoded literal");
    // mut_string.push("m"); // Error single quote
    mut_string.push('m');

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;

    // let str_from_substring: &str = &example_str[0..2]; // 0 is included and 2 is not
    // let str_from_substring: &str = &example_str[..2]; // if start from 0 we can omit zero
    // let str_from_substring: &str = &example_str[..]; // 0 to end of string
    // let str_from_substring: &str = &example_str[..=2]; // 2 is included
    // let str_from_substring: &str = &example_str[..200]; // This will ERROR

    // let char_by_index = &example_str[1]; // Error
    let char_by_index = &example_str.chars().nth(1); // Return Option

    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => {
            println!("None")
        }
    }

    // OR
    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}

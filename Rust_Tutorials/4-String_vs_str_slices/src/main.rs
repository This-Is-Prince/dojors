#[allow(unused_variables)]
fn main() {
    println!("Rust Strings");

    let example_str: &str = "Howdy";
    let example_string: String = String::from("Partner");

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_string;

    // let test = "first" + "second"; // error
    let combine_string_literals = ["first", "second"].concat();
    let combine_with_format_macro = format!("{} {}", "first", "second");

    // let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcoded literal");
    mut_string.push('m');
    // mut_string.push("m"); // error

    let a = String::from("A");
    let b = String::from("B");
    let combined = a + &b + &mut_string;

    // Sub strings
    // let str_from_substring: &str = &example_str[0..2]; // 0 to 1
    let str_from_substring: &str = &example_str[..2]; // 0 to 1
    let str_from_substring: &str = &example_str[..=2]; // 0 to 2
                                                       // let str_from_substring: &str = &example_str[..=200]; // This will NOT Give compile error but program panic

    let char_by_index = &example_str.chars().nth(1);

    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => {}
    }

    if let Some(v) = example_str.chars().nth(2) {
        println!("Found a char {}", v)
    }
}

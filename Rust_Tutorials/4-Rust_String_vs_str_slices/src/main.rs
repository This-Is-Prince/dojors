#[allow(unused_variables)]
fn main() {
    println!("------Rust String vs str (Slices)------");

    let example_str: &str = "Prince";
    let example_string: String = String::from("Prince");

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded string".to_string();

    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_string;

    // let test = "first" + "second"; // Error
    let combine_string_literals: String = ["first", "second"].concat();
    let combine_with_format_macro: String = format!("{} {}", "first", "second");

    let string_plus_str = example_string + example_str; // First 'String' than '&str'

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcoded literal");
    mut_string.push('a');
    // mut_string.push("m"); // Error

    let a = String::from("a");
    let b = String::from("b");
    let combined = a + &b + &mut_string;

    let str_from_substring: &str = &example_str[0..2];
    let str_from_substring: &str = &example_str[..2];
    let str_from_substring: &str = &example_str[..];
    let str_from_substring: &str = &example_str[1..];
    let str_from_substring: &str = &example_str[1..=2];

    // let char_by_index=&example_str[1]; // error
    let char_by_index = &example_str.chars().nth(1); // return Option<char>
    if let Some(ch) = char_by_index {
        println!("value of ch is : {}", ch);
    }

    match char_by_index {
        Some(ch) => println!("value of ch is : {}", ch),
        None => println!("no character"),
    }
}

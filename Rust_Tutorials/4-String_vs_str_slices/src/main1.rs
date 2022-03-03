#[allow(unused_variables)]
pub fn main1() {
    println!("---------Rust Strings---------");

    // Two types of strings in rust
    // 1. &str   (may be on stack, or hardcoded in binary)
    // 2. String (always allocated on heap)

    let example_str: &str = "Example of &str string";
    let example_string: String = String::from("Example of String string");
    println!("value of example_str is :- {}", example_str);
    println!("value of example_string is :- {}", example_string);

    // making strings in different ways
    let string_from_str: String = example_str.to_string();
    let string_from_hardcoded_str: String =
        "This is hardcoded &str string which is converted to `String`".to_string();

    println!("value of string_from_str is :- {}", string_from_str);
    println!(
        "value of string_from_hardcoded_str is :- {}",
        string_from_hardcoded_str
    );

    let string_from_hardcoded=String::from("Some hardcoded &str string passed to String::from function so that, this string can be converted to `String` string");
    let string_from_str_var = String::from(example_str);

    println!(
        "value of string_from_hardcoded is :- {}",
        string_from_hardcoded
    );
    println!("value of string_from_str_var is :- {}", string_from_str_var);

    // Converting String to &str, just write `&` in front of String variable
    let str_from_string: &str = &example_string;

    // Concatenating string
    let combine_string_literals: String = ["first", "second"].concat();
    let combine_with_format_macro: String = format!("{} {}", "first", "second");

    println!(
        "value of combine_string_literals is :- {}",
        combine_string_literals
    );
    println!(
        "value of combine_with_format_macro is :- {}",
        combine_with_format_macro
    );

    // using + operator, but first operand must be type String and rest of them must be &str
    let string_plus_str = String::from("First Operand must be Type String") + &example_str;
    let string_plus_str_more =
        combine_string_literals + &combine_with_format_macro + &string_plus_str;

    println!("value of string_plus_str is :- {}", string_plus_str);
    println!(
        "value of string_plus_str_more is :- {}",
        string_plus_str_more
    );

    // Creating Empty String
    let empty_string = String::new();
    println!("value of empty_string is :- {}", empty_string);
    // !!!!!REMINDER VARIABLES ARE ALWAYS IMMUTABLE IN RUST
    let mut mut_empty_string = String::new();
    println!(
        "Initial value of mut_empty_string is :- {}",
        mut_empty_string
    );
    mut_empty_string.push_str("push_str functions append this string to mut_empty_string");
    println!(
        "After Pushing a &str to mut_empty_string the value is :- {}",
        mut_empty_string
    );
    mut_empty_string.push('@'); // this function only push single character and must be in single quotes
                                // mut_empty_string.push("&"); // Error, character must be in single quotes
    println!(
        "After Pushing a single character to mut_empty_string the value is :- {}",
        mut_empty_string
    );

    // Slices, SubStrings
    let str_from_substring: &str = &example_str[0..=2];
    println!("Value of str_from_substring is :- {}", str_from_string);

    // Character by index
    let char_by_index: &str = &example_str[0..1]; // this is wrong type of char_by_index is &str not char
    println!("value of char_by_index is :- {}", char_by_index);
    let char_by_index: Option<char> = example_str.chars().nth(0); // copy of 0th character in example_str
    if let Some(v) = char_by_index {
        println!("value of char_by_index is :- {}", v);
    }
    let char_by_index: &mut Option<char> = &mut example_str.chars().nth(0); // reference of 0th character in example_str
    if let Some(ref mut v) = char_by_index {
        println!("value of char_by_index is :- {}", v);
        *v = '0';
        println!("value of char_by_index is :- {}", v);
    }
    if let Some(v) = char_by_index {
        println!("value of char_by_index is :- {}", v);
    }
    println!("{}", example_str);

    let my_string = String::from("First");
    let char_index = &mut my_string.chars().nth(0);
    println!("my_string :- {}", my_string);
    if let Some(ref mut v) = char_index {
        println!("value of char_index is :- {}", v);
        *v = '@';
        println!("value of char_index is :- {}", v);
    }
    println!("my_string :- {}", my_string);
}

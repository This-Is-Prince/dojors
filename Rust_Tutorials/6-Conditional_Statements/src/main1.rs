#[allow(unused_variables)]
pub fn main1() {
    println!("---------Conditional Statements---------");

    let some_bool = true;
    if some_bool {
        println!("some_bool is true...")
    }

    let some_bool = false;
    if !(some_bool != false) {
        println!("check is some_bool is false, if so we will not print this statement")
    }

    let int1 = 30;
    let int2 = 50;
    if !some_bool && (int1 < int2 && int2 < 100 && int1 > 10) {
        println!("if some_bool is false, int1 is less than int2, int2 is less than 100, int1 greater than 10 , after that this line will print...")
    }

    let var_from_inline = if int1 == 30 {
        30
    } else if int1 > 30 {
        31
    } else {
        29
    };

    println!("Value of var_from_inline is :- {}", var_from_inline);

    let some_bool = true;
    match some_bool {
        true => {
            println!("some_bool is true...")
        }
        false => {
            println!("some_bool is false...")
        }
    }
    which_ascii_character(50);
    which_ascii_character(69);
    which_ascii_character(111);
    which_ascii_character(40);
    which_ascii_character(91);
    which_ascii_character(123);
    which_ascii_character(93);
    which_ascii_character(125);
    which_ascii_character(41);
    which_ascii_character(2);
}

fn which_ascii_character(unsigned_integer: u8) {
    match unsigned_integer {
        capital_char_letters @ 65..=90 => {
            println!(
                "ASCII Codes of Capital Letters A to Z., ascii code {} : {}",
                capital_char_letters, capital_char_letters as char,
            )
        }
        small_char_letters @ 97..=122 => {
            println!(
                "ASCII Codes of small letters a to z., ascii code {} : {}",
                small_char_letters, small_char_letters as char,
            )
        }
        number @ 48..=57 => {
            println!(
                "ASCII Codes of 0 to 9., ascii code {} : {}",
                number, number as char
            )
        }
        opening_bracket @ (40 | 91 | 123) => {
            println!(
                "These are opening bracket :- {} : {}",
                opening_bracket, opening_bracket as char
            )
        }
        closing_bracket @ (41 | 93 | 125) => {
            println!(
                "These are closing bracket :- {} : {}",
                closing_bracket, closing_bracket as char
            )
        }
        code => {
            println!(
                "i don't know which character this code `{} : {}` belongs to",
                code, code as char
            );
        }
    }
}

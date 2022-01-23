#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    /*     print!("Tab\t Start with a tab\nand move to a new line"); */

    println!(
        "Inside quotes
you can write over
many lines
and it will print just fine."
    );

    println!(
        "If you forget to write
    on the left side, the spaces
    will be added when you print."
    );

    println!("Here are two escape characters: \\n and \\t");

    println!("He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file.");
    println!(
        r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
    );

    let my_string = "'Ice to see you,' he said.";
    let quote_string = r#""Ice to see you," he said."#;
    let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##;
    let many_hashtags =
        r####""You don't have to type ### to use a hashtag. You can just use #.""####;

    let r#let = 6;
    let mut r#mut = 10;
    r#mut = 1;

    let my_number = r#return();
    println!("{}", my_number);

    println!("{:?}", b"This will look like numbers");
    println!("{:?}", br##"I like to write. "#""##);

    println!("{:X}", '행' as u32);
    println!("{:X}", 'H' as u32);
    println!("{:X}", '居' as u32);
    println!("{:X}", 'い' as u32);

    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let number = 555;
    println!(
        "Binary: {:b}, hexadecimal: {:x}, octal: {:o}",
        number, number, number
    );

    let father_name = "Vlad";
    let son_name = "Adrian Fahrenheit";
    let family_name = "Țepeș";
    println!(
        "This is {1} {2}, son of {0} {2}.",
        father_name, son_name, family_name
    );

    println!(
        "{city1} is in {country} and {city2} is also in {country},
but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );
}

fn r#return() -> u8 {
    println!("Here is your number.");
    8
}

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    // let parsed_number = input.parse::<i32>()?; // Here is the question mark
    let parsed_number = input
        .parse::<u16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?; // Add a ? each time to check and pass it on
    Ok(parsed_number)
}

fn prints_three_things(vector: Vec<i32>) {
    if vector.len() != 3 {
        panic!("Vector must always have three items"); // will panic if the length is not 3
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

#[allow(dead_code)]
fn panic() {
    // panic!("Time to panic!");

    let my_vec = vec![8, 9, 10];
    prints_three_things(my_vec);
}

#[allow(dead_code)]
fn assertions() {
    let my_name = "Loki Laufeyson";
    assert!(my_name == "Loki Laufeyson");
    assert_eq!(my_name, "Loki Laufeyson");
    assert_ne!(my_name, "Mithridates");

    assert!(
        my_name == "Loki Laufeyson",
        "{} should be Loki Laufeyson",
        my_name
    );
    assert_eq!(
        my_name, "Loki Laufeyson",
        "{} and Loki Laufeyson should be equal",
        my_name
    );
    assert_ne!(
        my_name, "Mithridates",
        "You entered {}. Input must not equal Mithridates",
        my_name
    );

    /*
    let my_name = "Mithridates";
    assert_ne!(
        my_name, "Mithridates",
        "You enter {}. Input must not equal Mithridates",
        my_name
    );
    */
}

#[allow(dead_code)]
fn get_fourth(input: &Vec<i32>) -> i32 {
    // let fourth = input.get(3).unwrap();
    let fourth = input.get(3).expect("Input vector needs at least 4 items.");
    *fourth
}

fn main() {
    println!("The ? Operator.");

    if let Ok(number) = parse_str("5") {
        println!("The number is {}", number);
    }

    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
    panic();
    assertions();

    /*
    let my_vec = vec![9, 0, 10];
    let fourth = get_fourth(&my_vec);
    println!("Fourth is {}", fourth);
    */

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or(&0); // If .get doesn't work, we will make the value &0.
                                              // .get returns a reference, so we need &0 and not 0
                                              // You can write "let *fourth" with a * if you want fourth to be
                                              // a 0 and not a &0, but here we just print so it doesn't matter
    println!("{}", fourth);
}

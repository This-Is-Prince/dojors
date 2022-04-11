fn main() {
    println!("Copy Types");

    let my_number = 8;
    prints_number(my_number);
    prints_number(my_number);

    let country = String::from("Kiribati");
    prints_country_ownership(country);
    // prints_country_ownership(country); // ERROR, use of moved value: `country`

    let country = String::from("Kiribati");
    prints_country_clone(country.clone()); // make a clone and give it to the function, Only the clone goes in, and country is still alive
    prints_country_clone(country);

    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words "); // push the words on
        get_length_clone(my_string.clone()); // gives it a clone every time
    }

    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words "); // push the words on
        get_length_reference(&my_string); // gives it a clone every time
    }

    /* Variables without values */
    // let my_variables; // ERROR, type annotations needed

    let my_number;
    {
        // Pretend we need to have this code block
        let number = {
            // Pretend there is code here to make a number
            // Lots of code, and finally:
            57
        };

        my_number = loop_then_return(number);
    }

    println!("{}", my_number);
    let my_number;
    {
        my_number = 100;
    }
    println!("{}", my_number);
}

fn prints_number(number: i32) {
    // There is no -> so it's not returning anything
    // If number was not copy type, it would take it
    // and we could't use it again
    println!("{}", number);
}

fn prints_country_ownership(country_name: String) {
    println!("{}", country_name);
}

fn prints_country_clone(country_name: String) {
    println!("{}", country_name);
}

fn get_length_clone(input: String) {
    // Takes ownership of a String
    println!("It's {} words long.", input.split_whitespace().count()); // Splits to count the number of words
}

fn get_length_reference(input: &String) {
    // Takes ownership of a String
    println!("It's {} words long.", input.split_whitespace().count()); // Splits to count the number of words
}

fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

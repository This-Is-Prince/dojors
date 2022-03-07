fn prints_number(number: i32) {
    // there is no -> so it's not returning anything . IF number was not copy type, it would take it and we couldn't use it again
    println!("{}", number);
}

fn first() {
    let my_number = 8;
    prints_number(my_number); // Prints 8. prints_number gets a copy of my_number
    prints_number(my_number); // Prints 8 again.
                              // No problem, because my_number is copy type!
}

fn prints_country(country_name: String) {
    println!("{}", country_name);
}

fn second() {
    let country = String::from("Prince");
    prints_country(country); // move occurs because `country` has type `std::string::String`, which does not implement the `Copy` trait
                             // prints_country(country); // ERROR, use of moved value: `country`
}

fn third() {
    let country = String::from("Prince");
    prints_country(country.clone()); // cloning country string and move to prints_country as an arguments
    prints_country(country); // country still valid here
}

fn get_length_of_moved_string(input: String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn four() {
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words "); // push the words on
        get_length_of_moved_string(my_string.clone()); // gives it a clone every time
    }
}

fn get_length_of_reference_string(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn five() {
    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words "); // push the words on
        get_length_of_reference_string(&my_string);
    }
}

fn six() {
    // let my_variables; // Uninitialized variable.
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
fn seven() {
    let my_number;

    {
        // pretend we need to have this code block
        let number = {
            // pretend there is code here to make a number
            // lots of code, and finally:
            57
        };

        my_number = loop_then_return(number);
    }
    println!("{}", my_number);
}

fn eight() {
    let my_number;
    {
        my_number = 100;
    }
    println!("{}", my_number);
}

fn main() {
    println!("-----------Copy Types-----------");

    first();
    second();
    third();
    four();
    five();
    six();
    seven();
    eight();
}

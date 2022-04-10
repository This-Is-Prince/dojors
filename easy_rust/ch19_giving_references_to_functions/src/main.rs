fn main() {
    println!("Giving References to functions");

    let country = String::from("Australia");
    print_country_own(country); // We print "Australia"
                                /* print_country1(country); // ERROR, use of moved value: `country` */

    let country = String::from("Australia");
    let country = print_country_return(country); // we have to use let here now to get the String back
    print_country_return(country);

    let country = String::from("Australia");
    print_country_reference(&country); // We print "Australia";
    print_country_reference(&country); // That was fun, let's do it again!

    let mut country = String::from("Australia");
    add_new_zealand_mutable_reference(&mut country); // we also need to give it a mutable reference.

    let country = String::from("Australia"); // country is not mutable, but we are going to print Australia-NewZealand. How?
    add_new_zealand_mutable_own(country);
}

fn print_country_own(country_name: String) {
    println!("{}", country_name);
}

fn print_country_return(country_name: String) -> String {
    println!("{}", country_name);
    country_name // return it here
}

fn print_country_reference(country_name: &String) {
    println!("{}", country_name);
}

fn add_new_zealand_mutable_reference(country_name: &mut String) {
    // first we say that the function takes mutable reference
    country_name.push_str("-NewZealand"); // push_str() adds a &str to a String
    println!("Now it says: {}", country_name);
}

fn add_new_zealand_mutable_own(mut country: String) {
    // Here's how: add_new_zealand_mutable_own takes the String and declares it mutable!
    country.push_str("-NewZealand");
    println!("{}", country);
}

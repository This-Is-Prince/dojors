/*
fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(country);
    print_country(country); // Error
}
*/

/*
fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}

fn main() {
    let country = String::from("Austria");
    let country = print_country(country);
    print_country(country);
}
 */

/*
fn print_country(country_name: &String) {
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(&country);
    print_country(&country);
}
 */

/*
fn print_country(country_name: &mut String) {
    println!("{}", country_name);
    country_name.push_str("-Hungary");
}

fn main() {
    let mut country = String::from("Austria");
    print_country(&mut country);
    print_country(&mut country);
}
 */

fn print_country(mut country_name: String) {
    println!("{}", country_name);
    country_name.push_str("-Hungary");
    println!("{}", country_name);
}

fn main() {
    let country = String::from("Austria");
    print_country(country);
}

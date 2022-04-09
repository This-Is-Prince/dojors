#[allow(unused_variables)]
fn main() {
    println!("More on References");

    let country = String::from("Australia");
    let ref_one = &country;
    let ref_two = &country;
    println!("{}", ref_one);
    // let country = return_str();
    let country = return_str();
}
/*
fn return_str() -> &str // ERROR, expected named lifetime parameter
{
    let country = String::from("Australia");
    let country_ref = &country;
    country_ref
} */

#[allow(unused_variables)]
fn return_str() -> &'static str {
    let country = String::from("Australia");
    let country_ref = &country;
    "country_ref"
}

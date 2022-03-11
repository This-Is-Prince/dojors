#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    println!("-----------Lifetimes-----------");

    /* ================= Second ================= */
    /* let my_str = returns_reference();
    println!("{}", my_str) */

    /* ================= Third ================= */
    /* let my_str = returns_reference();
    println!("{}", my_str) */

    /* ================= Four ================= */
    /*     let my_city = City {
        name: "Prince",
        date_founded: 1921,
    }; */

    /* ================= Five ================= */
    /*     let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded); */

    /* ================= Six ================= */
    /*     let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()]; // city_names does not live for the whole program
    let my_city = City {
        name: &city_names[0], // This is a &str, but not a &'static str. It is a reference to a value inside city_names
        // borrowed value does not live long enough
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded); */

    /* ================= Seven ================= */
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}

/* ================= First ================= */
/* fn returns_reference() -> &str // ERROR, missing lifetime specifier, reference can't live long enough
{
    let my_string = String::from("I am a string");
    &my_string
}
 */

/* ================= Second ================= */
/* #[allow(unused_variables)]
fn returns_reference() -> &str // ERROR, missing lifetime specifier
{
    let my_string = String::from("I am a string");
    "I am a str"
}
 */

/* ================= Third ================= */
/* #[allow(unused_variables)]
fn returns_reference() -> &'static str {
    let my_string = String::from("I am a string");
    "I am a str"
}
 */

/* ================= Four ================= */
/* #[derive(Debug)]
struct City {
    name: &str, // ERROR, missing lifetime specifier
    date_founded: u32,
} */

/* ================= Five ================= */
/* #[allow(dead_code)]
#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
} */

/* ================= Six ================= */
/* #[allow(dead_code)]
#[derive(Debug)]
struct City {
    name: &'static str, // must live for the whole program
    date_founded: u32,
} */

/* ================= Seven ================= */
/* #[allow(dead_code)]
#[derive(Debug)]
struct City<'a>
// City has lifetime 'a.
{
    name: &'a str, // and name also has lifetime 'a.
    date_founded: u32,
} */

#[allow(dead_code)]
#[derive(Debug)]
struct City<'city>
// City has lifetime 'city.
{
    name: &'city str, // and name also has lifetime 'city.
    date_founded: u32,
}

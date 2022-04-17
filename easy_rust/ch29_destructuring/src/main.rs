#[allow(dead_code)]
struct Person {
    // make a simple struct for a person
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

#[allow(dead_code)]
struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        Self {
            name,
            date_founded,
            name_before,
            population,
        }
    }
}

fn process_city_values(city: &City) {
    #[allow(unused_variables)]
    let City {
        name,
        name_before,
        population,
        date_founded,
    } = city;
    // now we have the values to use separately
    let two_names = vec![name, name_before];
    println!("The city's two names are {:?}", two_names);
}

#[allow(dead_code)]
fn main() {
    println!("Destructuring");

    let papa_doc = Person {
        // create variable papa_doc
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };

    let Person {
        // Destructure papa_doc
        name: a,
        real_name: b,
        height: c,
        happiness: d,
    } = papa_doc;

    println!(
        "They cal him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        a, b, c, d
    );

    let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
    process_city_values(&tallinn);
}

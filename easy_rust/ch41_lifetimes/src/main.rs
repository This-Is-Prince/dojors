// #[allow(dead_code)]
// #[derive(Debug)]
// struct City<'a> {
//     // City has lifetime 'a
//     name: &'a str, // and name also has lifetime 'a.
//     date_founded: u32,
// }

// #[allow(dead_code)]
// struct Adventurer<'a> {
//     name: &'a str,
//     hit_points: u32,
// }

// #[allow(dead_code)]
// impl Adventurer<'_> {
//     fn take_damage(&mut self) {
//         self.hit_points -= 20;
//         println!("{} has {} hit points left!", self.name, self.hit_points);
//     }
// }

// impl std::fmt::Display for Adventurer<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} has {} hit points.", self.name, self.hit_points)
//     }
// }

// fn main() {
//     println!("Lifetimes");

//     let my_str = returns_str();
//     println!("{}", my_str);

//     let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
//     let my_city = City {
//         name: &city_names[0],
//         date_founded: 1921,
//     };
//     println!("{} was founded in {}", my_city.name, my_city.date_founded);

//     let mut billy = Adventurer {
//         name: "Billy",
//         hit_points: 100_000,
//     };
//     println!("{}", billy);
//     billy.take_damage();
// }

// /*
// fn returns_str() -> &str {
//     //missing lifetime specifier
//     let my_string = String::from("I am a String");
//     "I am a str"
// }
// */
// #[allow(unused_variables)]
// fn returns_str() -> &'static str {
//     let my_string = String::from("I am a String");
//     "I am a str"
// }

#[derive(Debug)]

struct Person<'a, 'b> {
    name: &'a str,
    designation: &'b str,
}

impl<'a> Person<'a, '_> {
    fn change_name(&mut self, name: &'a str){
        self.name = name;
    }
}

fn main() {
    println!("Hello World");
    let name = String::from("Prince");
    let designation = String::from("Web Developer");
    let mut person = Person {
        name: &name,
        designation: &designation,
    };
    println!(
        "{:#?}, name: {:#?}, designation: {:#?}",
        person, person.name, person.designation
    );
    let name1 = String::from("Prince Kumar");
    person.change_name(&name1);
    println!(
        "{:#?}, name: {:#?}, designation: {:#?}",
        person, person.name, person.designation
    );
}

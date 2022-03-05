#[allow(dead_code)]
struct Person {
    name: String,
}

impl Person {
    // fn new(name: &str) -> Self {
    //     Self {
    //         name: name.to_string(),
    //     }
    // }

    fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

#[allow(unused_variables)]
fn into() {
    // let tyrion = Person::new("Tyrion");
    // let name = "Jane".to_string();
    // let jane = Person::new(name.as_ref());

    let tyrion = Person::new("Tyrion");
    let cersi = Person::new("Cersi");
}

fn main() {
    println!("-----------Into-----------");

    into();
}

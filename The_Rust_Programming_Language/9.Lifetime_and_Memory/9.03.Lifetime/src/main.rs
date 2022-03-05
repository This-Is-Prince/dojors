#[allow(dead_code)]
#[allow(dead_code)]
struct Person {
    name: String,
}

#[allow(dead_code)]
impl Person {
    // fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

#[allow(dead_code)]
struct Company<'a> {
    name: String,
    ceo: &'a Person,
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    println!("----------Lifetime----------");

    let boss = Person {
        name: String::from("Prince"),
    };
    let my_company = Company {
        name: String::from("My Company"),
        ceo: &boss,
    };

    let z: &String;
    {
        let p = Person {
            name: String::from("Tywin"),
        };
        z = p.get_ref_name();
        println!("z = {}", z);
    }
}

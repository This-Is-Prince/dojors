#[allow(unused_imports)]
use std::rc::Rc;

#[allow(dead_code)]
struct Person {
    // name: String,
    name: Rc<String>,
}

#[allow(dead_code)]
impl Person {
    // fn new(name: String) -> Self {
    //     Self { name }
    // }
    fn new(name: Rc<String>) -> Self {
        Self { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {} Thank You.", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn rc_demo() {
    // 1.First
    // let name = "John".to_string();
    // let person = Person::new(name);

    // person.greet();
    // println!("Name = {}", name); // ERROR, borrow of moved value: `name`

    // 2.Second
    // let name = Rc::new("John".to_string());
    // let person = Person::new(name.clone()); // It is not actually cloned, but it increment reference count of name variable

    // person.greet();
    // println!("Name = {}", name);

    let name = Rc::new("John".to_string());
    println!(
        "Name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name)
    );
    {
        let person = Person::new(name.clone());
        println!(
            "Name = {}, name has {} strong pointers",
            name,
            Rc::strong_count(&name)
        );
        person.greet();
    }
    println!(
        "Name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name)
    );
}

fn main() {
    println!("------------Reference-Counted Variables------------");

    rc_demo();
}

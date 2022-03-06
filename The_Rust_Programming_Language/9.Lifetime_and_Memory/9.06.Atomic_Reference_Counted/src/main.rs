#[allow(unused_imports)]
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[allow(dead_code)]
struct Person {
    name: Arc<String>,
}

#[allow(dead_code)]
impl Person {
    fn new(name: Arc<String>) -> Self {
        Self { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {} Thank You.", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn arc_demo() {
    let name = Arc::new("John".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);

    t.join().unwrap();
}

fn main() {
    println!("------------Atomic Reference-Counted Variables------------");

    arc_demo();
}

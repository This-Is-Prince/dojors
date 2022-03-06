#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}, Thank You.", self.name);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    println!("--------------Lifetime In Structure Implementation--------------");

    let person = Person { name: "Prince" };
    person.talk();
}

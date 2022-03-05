trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Self {
        Self { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Self {
        Self { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

fn traits() {
    let h = Human { name: "John" };
    h.talk();
    let c = Cat { name: "Sansa" };
    c.talk();

    let h1 = Human::create("JOHN");
    h1.talk();
    let c1 = Cat::create("Arya");
    c1.talk();

    // let h = Animal::create("JOHN"); // ERROR, type annotations needed
    let h: Human = Animal::create("JOHN");
    h.talk();
    let c: Cat = Animal::create("Cersi");
    c.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}

fn main() {
    println!("-----------Traits-----------");
    traits();
}

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Self {
        println!("{} enters the game", name);
        Self { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

#[allow(unused_variables)]
fn drop1() {
    // let goblin = Creature::new("Jeff");
    // println!("game proceeds");
    // // goblin.drop(); // ERROR, explicit destructor calls not allowed
    // drop(goblin);

    let clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        clever = goblin;
        println!("end of scope");
    }
    println!("{}", clever.name);
}

fn main() {
    println!("----------Drop----------");

    drop1();
}

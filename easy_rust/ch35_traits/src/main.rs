/* use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    number: usize,
}

#[allow(dead_code)]
struct ThingsToAdd {
    first_thing: u32,
    second_thing: f32,
}
#[allow(dead_code)]

struct Animal {
    // A simple struct - an Animal only has a name
    name: String,
}

trait Dog {
    // The dog trait gives some functionality
    /*     fn bark(&self) {
        // It can bark
        println!("Woof woof!");
    }
    fn run(&self) {
        // and it can run
        println!("The dog is running!");
    } */

    fn bark(&self); // bark() says it needs a &self and returns nothing
    fn run(&self); // run() says it needs a &self and returns nothing.
                   // So now we have to write them ourselves.
}

impl Dog for Animal {
    // Now Animal has the trait Dog
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn print_cats(pet: String) {
    println!("{}", pet);
}

#[allow(dead_code)]
struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.longitude, self.latitude)
    }
}
 */

struct Monster {
    health: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Wizard {
    // Now Wizard has Debug
    health: i32, // Now Wizard has health
}

#[allow(dead_code)]
#[derive(Debug)]
struct Ranger {
    // So does Ranger
    health: i32, // So does Ranger
}

trait FightClose: std::fmt::Debug {
    // Now a type needs Debug to use FightClose
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health,&self
        ); // We can now print self with {:?} because we have Debug
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your hand. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, &self
        );
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance: std::fmt::Debug {
    // We could also do trait FightFromDistance:FightClose because FightClose needs Debug
    fn attach_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow. Your opponent now has {} health left. You are now at: {:?}",
                opponent.health,self
            );
        }
    }
    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }
        println!(
            "You attack with your rock. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightFromDistance for Ranger {}

fn main() {
    println!("Traits");

    /*
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark(); // Now Animal can use bark()
    rover.run(); // and it can use run()

    let mr_mangle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!("Mr. Mantle is a {:?}", mr_mangle);
    println!("{}", mr_mangle);
    print_cats(mr_mangle.to_string()); // Turn him into a String here
    println!(
        "Mr. Mantle's String is {} letters long.",
        mr_mangle.to_string().chars().count()
    ); // Turn him into chars and count them
    */

    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attach_with_bow(&mut uruk_hai, 8);
}

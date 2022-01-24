/* =========1========= */
/*
#[allow(dead_code)]
#[derive(Debug)]
struct MyStruct {
    number: usize,
}
fn main() {}
*/

/* =========2========= */
/* #[allow(dead_code)]
struct Animal {
    name: String,
}
trait Dog {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is running!");
    }
}
impl Dog for Animal {}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
}
*/

/* =========3========= */
/*
struct Animal {
    name: String,
}
trait Dog {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is running!");
    }
}
impl Dog for Animal {
    fn run(&self) {
        println!("{} is running!.", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
}
 */

/* =========4========= */
/* use std::fmt;
struct Cat {
    name: String,
    age: u8,
}
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}
fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    println!("{}", mr_mantle);
}
 */

/* =========5========= */
/* use std::fmt;
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

fn main() {
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };
    print_cats(mr_mantle.to_string());
    println!("{}", mr_mantle);
}
 */

/* =========6========= */
/*
struct Monster {
    health: i32,
}
struct Wizard {}

struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent now has {} health left.",
            opponent.health
        );
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your hand. Your opponent now has {} health left.",
            opponent.health
        );
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "You attack with your bow. Your opponent now has {} health left.",
                opponent.health
            );
        }
    }
    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
        }
        println!(
            "You attack with your rock. Your opponent now has {} health left.",
            opponent.health
        );
    }
}

impl FightFromDistance for Ranger {}

fn main() {
    let harry_potter = Wizard {};
    let jon_snow = Ranger {};
    let mut voldemort = Monster { health: 40 };
    harry_potter.attack_with_sword(&mut voldemort);
    jon_snow.attack_with_bow(&mut voldemort, 8);
}
 */

/* =========7========= */
/* use std::fmt;
struct Monster {
    health: i32,
}
#[allow(unused)]
#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[allow(unused)]
#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait FightClose: fmt::Debug {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health,self
        );
    }

    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your hand. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, self
        );
    }
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}

trait FightFromDistance: fmt::Debug {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
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
    let harry_potter = Wizard { health: 60 };
    let jon_snow = Ranger { health: 80 };
    let mut voldemort = Monster { health: 40 };
    harry_potter.attack_with_sword(&mut voldemort);
    jon_snow.attack_with_bow(&mut voldemort, 8);
}
 */

/* =========8========= */
use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[allow(unused)]
#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[allow(unused)]
#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}
impl Magic for Wizard {}

fn attack_with_bow<T: FightFromDistance + Debug>(
    character: &T,
    opponent: &mut Monster,
    distance: u32,
) {
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, character
        );
    }
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character
    );
}

fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, character
        );
    }
}

fn main() {
    let harry_potter = Wizard { health: 60 };
    let jon_snow = Ranger { health: 80 };

    let mut night_king = Monster { health: 40 };

    attack_with_sword(&harry_potter, &mut night_king);
    attack_with_bow(&jon_snow, &mut night_king, 8);
    fireball(&harry_potter, &mut night_king, 8);
}

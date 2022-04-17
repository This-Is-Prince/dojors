#[allow(dead_code)]
#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[allow(dead_code)]
impl Animal {
    fn new() -> Self {
        // Self means Animal.
        // You can also write Animal instead of Self
        Self {
            // When we write Animal::new(), we always get a cat that is 10 years old
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        // Because we are inside Animal, &mut self means &mut Animal
        // use .change_to_dog() to change the cat to dog
        // with &mut self we can change it
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        // use .change_to_cat() to change the dog to cat
        // with &mut self we can change it
        println!("Changing animal to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        // we want to read self
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}

fn main() {
    println!("Implementing Structs and Enums");

    let mut new_animal = Animal::new(); // Associated function to create a new animal
                                        // It is a cat, 10 years old

    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();

    let my_mood = Mood::Sleepy;
    my_mood.check();
}

#[allow(dead_code)]
enum Mood {
    Good,
    Bad,
    Sleepy,
}

impl Mood {
    fn check(&self) {
        match self {
            Mood::Good => println!("Feeling good!"),
            Mood::Bad => println!("Eh, not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}

const MAX_HEALTH: u32 = 100;

#[allow(dead_code)]
#[derive(Debug)]
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
}

#[allow(dead_code)]
#[derive(Debug)]
enum PlayerAction {
    Attack,
    Heal(u32),
    Wait,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    class: CharacterClass,
    is_active: bool,
}

impl Character {
    fn new(name: String, class: CharacterClass) -> Self {
        Self {
            name,
            health: MAX_HEALTH,
            class,
            is_active: true,
        }
    }

    fn display_status(&self) {
        println!("--- Status Report ---");
        println!("Name: {}", self.name);
        println!("Class: {:?}", self.class);
        println!("Health: {}/{}", self.health, MAX_HEALTH);
        println!("---------------------");
    }

    fn perform_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Attack => {
                println!("{} attacks furiously!", self.name);
            }
            PlayerAction::Heal(amount) => {
                self.health += amount;

                if self.health > MAX_HEALTH {
                    self.health = MAX_HEALTH;
                }
                println!("{} heals for {} points. Current health: {}", self.name, amount, self.health)
            }
            PlayerAction::Wait => {
                println!("{} waits, gathering their strength.", self.name)
            }
        }
    }
}

fn main() {
    let mut hero = Character::new(String::from("Arion"), CharacterClass::Warrior);
    hero.display_status();

    let mut turn_count = 0;

    loop {
        turn_count += 1;
        println!("\n--- Turn {} ---", turn_count);

        let current_action = if hero.health < 50 {
            PlayerAction::Heal(20)
        } else {
            PlayerAction::Attack
        };

        hero.perform_action(current_action);

        if let CharacterClass::Mage = hero.class {
            println!("As a Mage, {} finds a hidden mana potion!", hero.name);
        }

        if turn_count >= 3 {
            println!("Simulation finished.");
            break;
        }
    }

    hero.display_status();
}
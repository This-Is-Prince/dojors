mod items;
mod shop;

use shop::Shop;
use items::{Item, ItemType};
use std::io;

fn main() {
    let mut my_shop = Shop::new(100);

    let _ = my_shop.add_item(Item::new(String::from("Iron Sword"), ItemType::Weapon, 25, 3));
    let _ = my_shop.add_item(Item::new(String::from("Health Potion"), ItemType::Potion, 10, 10));
    let _ = my_shop.add_item(Item::new(String::from("Chainmail Armor"), ItemType::Armor, 80, 2));

    loop {
        my_shop.print_inventory();

        println!("\n What would you like to do?");
        println!("1. Sell an item");
        println!("2. Inspect front counter");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Which item to sell? (Type the exact name)");
                let mut item_to_sell = String::new();
                io::stdin().read_line(&mut item_to_sell).expect("Failed to read line");

                match my_shop.sell_item(item_to_sell.trim()) {
                    Ok(price) => println!("Sale complete! Earned {} gold.", price),
                    Err(e) => println!("Error: {}", e),
                }
            },
            "2" => {
                my_shop.inspect_front_counter();
            },
            "3" => {
                println!("Thank you for your business!");
                break;
            },
            _ => {
                println!("Invalid choice, please enter 1, 2, or 3.");
            }
        }
    }
}

#[allow(dead_code)]
fn main_old() {
    let shop_name = "The Rusty Potion";
    println!("Welcome to {}!", shop_name);

    let mut my_shop = Shop::new(100);
    println!("Our shop has {} gold.", my_shop.gold);

    my_shop.gold += 50;
    println!("A customer paid! Now we have {} gold.", my_shop.gold);

    let item_price: u32 = 25;
    let is_magical: bool = true;
    let initial_letter: char = 'S';

    let sword_name = String::from("Dragon's Tooth");

    let potion_name: &str = "Elixir of Health";

    println!("{}, {}, {}, {}, {}", item_price, is_magical, initial_letter, sword_name, potion_name);

    let potion_info = (String::from("Greater Healing Potion"), 50);

    let (name, price) = potion_info;

    println!("The tuple contained: {} which costs {}.", name, price);
}

#[allow(dead_code)]
fn run_shop_business() {
    let mut my_shop = Shop::new(100);

    let item1 = Item::new(String::from("Iron Sword"), ItemType::Weapon, 25, 1);
    let item2 = Item::new(String::from("Leather Armor"), ItemType::Armor, 40, 1);

    my_shop.add_item(item1).unwrap();

    my_shop.add_item(item2).unwrap();
    println!("Successfully added items.");
}
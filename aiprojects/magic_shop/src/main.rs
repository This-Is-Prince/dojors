mod items;
mod shop;

use shop::Shop;
use items::{Item, ItemType};

fn main() {
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

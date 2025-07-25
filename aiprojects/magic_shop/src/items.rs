#[derive(Debug)]
pub enum ItemType {
    Weapon,
    Armor,
    Potion,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub price: u32,
    pub quantity: u32,
}

impl Item {
    pub fn new(name: String, item_type: ItemType, price: u32, quantity: u32) -> Self {
        Self {
            name,
            item_type,
            price,
            quantity,
        }
    }

    pub fn print_details(&self) {
        println!("- {}: A {:?} worth {} gold. We have {}.", self.name, self.item_type, self.price, self.quantity);
    }
}
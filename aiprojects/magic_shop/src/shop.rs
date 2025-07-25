use crate::items::Item;

const MAX_INVENTORY_SLOTS: usize = 10;

pub struct Shop {
    pub inventory: [Option<Item>; MAX_INVENTORY_SLOTS],
    pub gold: u32,
}

// struct ShopLocation(i32, i32);
// struct ClosedForBusiness;

impl Shop {
    pub fn new(starting_gold: u32) -> Self {
        Self {
            inventory: std::array::from_fn(|_| None),
            gold: starting_gold
        }
    }

    pub fn add_item(&mut self, item_to_add: Item) -> Result<(), String> {
        for slot in self.inventory.iter_mut() {
            if slot.is_none() {
                *slot = Some(item_to_add);
                println!("Added {} to the shop's inventory.", slot.as_ref().unwrap().name);
                return Ok(());
            }
        }

        Err(String::from("Inventory is full!"))
    }

    pub fn print_inventory(&self) {
        println!("\n--- Shop Inventory ---");
        println!("Gold: {}", self.gold);

        for slot in self.inventory.iter() {
            match slot {
                Some(item) => item.print_details(),
                None => (),
            }
        }

        println!("-----------------\n");
    }

    pub fn sell_item(&mut self, item_name: &str) -> Result<u32, String> {
        for slot in self.inventory.iter_mut() {
            if let Some(item) = slot.as_mut() {
                if item.name == item_name {
                    if item.quantity > 0 {
                        let price = item.price;
                        item.quantity -= 1;
                        self.gold += price;

                        println!("Sold {} for {} gold!", item_name, price);

                        if item.quantity == 0 {
                            *slot = None;
                        }

                        return Ok(price);
                    } else {
                        return Err(format!("{} is out of stock!", item_name));
                    }
                }
            }
        }
        Err(format!("Item '{}' not found in inventory.", item_name))
    }

    pub fn inspect_front_counter(&self) {
        let front_counter_slice: &[Option<Item>] = &self.inventory[0..3];

        println!("\n--- Inspecting Front Counter ---");
        for slot in front_counter_slice {
            match slot {
                Some(item) => println!("On display: {}", item.name),
                None => println!("[Empty Display Stand]"),
            }
        }
        println!("-------------------");
    }
}
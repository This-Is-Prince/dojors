mod finance;
mod inventory;

use inventory::{Item, StockStatus};
use finance::{Money};

const STORE_NAME: &str = "RustMart";

fn generate_stock_report(inventory: &[Item]) {
    println!("\n--- {} Stock Report ---", STORE_NAME);

    for item in inventory {
        item.display();
        if let StockStatus::LowStock(qty) = item.status {
            println!("  -> WARNING: Low stock, only {} left!", qty);
        }
    }
    println!("--- End of Report ---");
}

fn main() {
    let welcome_message: String = String::from("Welcome to the Inventory System!");
    println!("{}", welcome_message);

    let mut inventory: [Item; 3] = [
        Item::new(String::from("Laptop"), Money(1200.00), 15),
        Item::new(String::from("Mouse"), Money(25.00), 8),
        Item::new(String::from("Keyboard"), Money(75.50), 0),
    ];

    let transaction_log: (u32, &str, f64) = (1, "Initial setup", 0.0);
    let (id, action, amount) = transaction_log;
    println!("Initial Log: ID #{}, Action: '{}', Amount: ${}", id, action, amount);

    generate_stock_report(&inventory);

    let laptop = &mut inventory[0];
    laptop.sell();

    let item_to_find = "Mouse";
    let found_item = 'search: loop {
        for item in &inventory {
            if item.name == item_to_find {
                break 'search Some(item);
            }
        }

        break 'search None;
    };

    let Some(item) = found_item else {
        panic!("This should not happen in this example, item exists.");
    };

    println!("\n Found item via loop: {:?}", item);

    generate_stock_report(&inventory);
}

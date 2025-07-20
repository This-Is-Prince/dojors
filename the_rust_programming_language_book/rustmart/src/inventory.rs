use super::finance::Money;

#[derive(Debug)]
pub enum StockStatus {
    InStock,
    LowStock(u32),
    OutOfStock,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub price: Money,
    pub quantity: u32,
    pub status: StockStatus,
}

impl Item {
    pub fn new(name: String, price: Money, quantity: u32) -> Self {
        let status = if quantity == 0 {
            StockStatus::OutOfStock
        } else if quantity < 10 {
            StockStatus::LowStock(quantity)
        } else {
            StockStatus::InStock
        };

        Self {
            name,
            price,
            quantity,
            status,
        }
    }

    pub fn display(&self) {
        println!("- Item: {}, Price: ${:.2}, Quantity: {}", self.name, self.price.0, self.quantity)
    }

    pub fn sell(&mut self) {
        if self.quantity > 0 {
            self.quantity -= 1;
            println!("Sold one {}. Remaining: {}", self.name, self.quantity);

            self.status = match self.quantity {
                0 => StockStatus::OutOfStock,
                1..=9 => StockStatus::LowStock(self.quantity),
                _ => StockStatus::InStock
            }
        } else {
            println!("Cannot sell {}. It's out of stock.", self.name);
        }
    }
}
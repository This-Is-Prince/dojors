use crate::items::Item;

const MAX_INVENTORY_SLOTS: usize = 10;

pub struct Shop {
    pub inventory: [Option<Item>; MAX_INVENTORY_SLOTS],
    pub gold: u32,
}

struct ShopLocation(i32, i32);
struct ClosedForBusiness;

impl Shop {
    pub fn new(starting_gold: u32) -> Self {
        Self {
            inventory: [None; MAX_INVENTORY_SLOTS],
            gold: starting_gold,
        }
    }
}
/* struct Item {
    number: u8,
}

fn main() {
    /*     let my_number = 9;
    let reference = &my_number;
    println!("{}", my_number == reference); // Error &u8 == u8 */
    /*
    let item = Item { number: 8 };
    let reference_number = &item.number;
    println!("{}", reference_number == 8); // Error &u8 == u8 */

    let item = Item { number: 8 };
    let reference_item = &item;
    println!("{}", reference_item.number == 8); // true
}
 */

struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!(
            "Are {} and {} equal? {}",
            self.number,
            other_number,
            self.number == other_number
        );
    }
}

fn main() {
    let item = Item { number: 8 };
    let reference_item = &item;
    let reference_item_two = &reference_item;

    item.compare_number(8); //true
    reference_item.compare_number(8); //true
    reference_item_two.compare_number(8); //true
}

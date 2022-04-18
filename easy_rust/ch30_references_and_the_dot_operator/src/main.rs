struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        // takes a reference to self
        println!(
            "Are {} and {} equal? {}",
            self.number,
            other_number,
            self.number == other_number
        );
        // We don't need to write *self.number
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    println!("References and the dot operator");

    let my_number = 9;
    let reference = &my_number;
    // println!("{}", my_number == reference); // ERROR, can't compare `{integer}` with `&{integer}`
    println!("{}", my_number == *reference);

    let item = Item { number: 8 };
    let reference_number = &item.number; // reference number type is &u8
                                         // println!("{}", reference_number == 8); // ERROR, can't compare `&u8` with `{integer}`
    println!("{}", *reference_number == 8);

    let reference_item = &item;
    println!("{}", (*reference_item).number == 8);
    println!("{}", reference_item.number == 8); // we don't need to write *reference_item.number

    let item = Item { number: 8 };
    let reference_item = &item; // This is type &Item
    let reference_item_two = &reference_item; // This is type &&Item
    item.compare_number(8); // this method works
    reference_item.compare_number(8); // it works here too
    reference_item_two.compare_number(8); // and here
}

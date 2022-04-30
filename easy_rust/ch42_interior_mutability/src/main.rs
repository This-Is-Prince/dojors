use std::cell::{Cell, RefCell};

#[derive(Debug)]
#[allow(dead_code)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

fn main() {
    println!("Interior Mutability");

    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
    println!("{:#?}", super_phone_3000);
    // 10 year later, super_phone_3000 is not on sale anymore
    super_phone_3000.on_sale.set(false);
    println!("{:#?}", super_phone_3000);

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };
    println!("{:#?}", user_1);
    println!("{:#?}", user_1.active);
    user_1.active.replace(false);
    println!("{:#?}", user_1.active);
    user_1.active.replace_with(|x| !*x);
    println!("{:#?}", user_1.active);

    let date = 2020;
    user_1
        .active
        .replace_with(|_| if date < 2000 { true } else { false });
    println!("{:#?}", user_1.active);
}

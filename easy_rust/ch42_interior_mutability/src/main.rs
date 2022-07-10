// use std::cell::{Cell, RefCell};

// #[derive(Debug)]
// #[allow(dead_code)]
// struct PhoneModel {
//     company_name: String,
//     model_name: String,
//     screen_size: f32,
//     memory: usize,
//     date_issued: u32,
//     on_sale: Cell<bool>,
// }

// #[derive(Debug)]
// #[allow(dead_code)]
// struct User {
//     id: u32,
//     year_registered: u32,
//     username: String,
//     active: RefCell<bool>,
//     // Many other fields
// }

// fn main() {
//     println!("Interior Mutability");

//     let super_phone_3000 = PhoneModel {
//         company_name: "YY Electronics".to_string(),
//         model_name: "Super Phone 3000".to_string(),
//         screen_size: 7.5,
//         memory: 4_000_000,
//         date_issued: 2020,
//         on_sale: Cell::new(true),
//     };
//     println!("{:#?}", super_phone_3000);
//     // 10 year later, super_phone_3000 is not on sale anymore
//     super_phone_3000.on_sale.set(false);
//     println!("{:#?}", super_phone_3000);

//     let user_1 = User {
//         id: 1,
//         year_registered: 2020,
//         username: "User 1".to_string(),
//         active: RefCell::new(true),
//     };
//     println!("{:#?}", user_1);
//     println!("{:#?}", user_1.active);
//     user_1.active.replace(false);
//     println!("{:#?}", user_1.active);
//     user_1.active.replace_with(|x| !*x);
//     println!("{:#?}", user_1.active);

//     let date = 2020;
//     user_1
//         .active
//         .replace_with(|_| if date < 2000 { true } else { false });
//     println!("{:#?}", user_1.active);
// }

use std::{
    cell::{Cell, RefCell},
    sync::{Mutex, RwLock},
};
#[allow(dead_code)]

struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    // 10 year later, super_phone_3000 is not on sale anymore
    super_phone_3000.on_sale.set(false);

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{:?}", user_1.active);

    user_1.active.replace(false);
    println!("{:?}", user_1.active);

    let date = 2020;
    user_1
        .active
        .replace_with(|_| if date < 2000 { true } else { false });
    println!("{:?}", user_1.active);

    // let borrow_one = user_1.active.borrow_mut();
    // let borrow_two = user_1.active.borrow_mut();

    let my_mutex = Mutex::new(5);
    println!("{:?}", my_mutex);
    // {
    //     let mut mutex_changer = my_mutex.lock().unwrap();
    //     println!("{:?}", my_mutex);
    //     println!("{:?}", mutex_changer);
    //     *mutex_changer = 6;
    //     println!("{:?}", mutex_changer);
    // } // mutex_changer goes out of scope - now it is gone. It is not locked anymore

    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("{:?}", my_mutex); // Now it says: Mutex { data: 6 }
    *mutex_changer = 6;
    println!("{:?}", mutex_changer);
    std::mem::drop(mutex_changer); // drop mutex_changer - it is gone now
                                   // and my_mutex is unlocked
    println!("{:?}", my_mutex); // Now it says: Mutex { data: 6 }

    // let my_mutex = Mutex::new(5);
    // let mut mutex_changer = my_mutex.lock().unwrap();
    // let mut other_mutex_changer = my_mutex.lock().unwrap();

    // println!("This will never print...");

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();

    if let Ok(value) = other_mutex_changer {
        println!("The MutexGuard has: {}", value);
    } else {
        println!("Didn't get the lock");
    }

    let my_mutex = Mutex::new(5);
    println!("{:?}", my_mutex);
    *my_mutex.lock().unwrap() = 6;
    println!("{:?}", my_mutex);

    let my_mutex = Mutex::new(0);
    println!("{:?}", my_mutex);
    for _ in 1..=100 {
        *my_mutex.lock().unwrap() += 1;
    }
    println!("{:?}", my_mutex);

    // RWLock
    // let my_rwlock = RwLock::new(5);

    // let read1 = my_rwlock.read().unwrap(); // one .read() is fine
    // let read2 = my_rwlock.read().unwrap(); // two .read() is also fine

    // println!("{:?}, {:?}", read1, read2);
    // let write1 = my_rwlock.write().unwrap(); // uh oh, now the program will wait forever

    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    println!("{:?}, {:?}", read1, read2);

    drop(read1);
    drop(read2); // we dropped both, so we can use .write() now

    println!("{:?}", my_rwlock);
    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    println!("{:?}", my_rwlock);
    drop(write1);
    println!("{:?}", my_rwlock);

    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!");
    };
}

// RefCell
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
    // Many other fields
}

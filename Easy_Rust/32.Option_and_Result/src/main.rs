/* ========1======== */
/* fn take_fifth(value: Vec<i32>) -> i32 {
    value[4]
}
fn main() {
    let new_vec = vec![1, 2, 3];
    let index = take_fifth(new_vec);
    println!("{}", index);
} */

/* ========2======== */
/* fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}
fn main() {
    let new_vec = vec![1, 2, 3];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}", take_fifth(new_vec), take_fifth(bigger_vec));
}
 */

/* ========3======== */
/*
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}
fn main() {
    let new_vec = vec![1, 2, 3];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!(
        "{:?}, {:?}",
        take_fifth(new_vec).unwrap(),
        take_fifth(bigger_vec).unwrap()
    );
}
 */

/* ========4======== */
/* fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_option(my_option: Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {}!", number),
            None => println!("Found a None!"),
        }
    }
}

fn main() {
    let new_vec = vec![1, 2, 3];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new();

    option_vec.push(take_fifth(new_vec));
    option_vec.push(take_fifth(bigger_vec));

    handle_option(option_vec);
}
 */

/* ========5======== */
/* fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1, 2, 3];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap());
        } else {
            println!("We got nothing.");
        }
    }
}
 */

/* ========6======== */
/* fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}
fn main() {
    if give_result(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }
}
 */

/* ========7======== */
/*
fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()),
    }
}
fn main() {
    let mut result_vec = Vec::new();
    for number in 2..7 {
        result_vec.push(check_if_five(number));
    }
    println!("{:?}", result_vec);
}
 */

/* ========8======== */
/* fn main() {
    let my_vec = vec![2, 3, 4];
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
    }
}
 */

/* ========9======= */

fn main() {
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);
        while let Some(information) = city.pop() {
            if let Ok(number) = information.parse::<i32>() {
                println!("The number is: {}", number);
            }
        }
    }
}

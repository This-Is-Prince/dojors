/* fn take_fifth_without_option(value: Vec<i32>) -> i32 {
    value[4]
} */

fn take_fifth_with_option(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        // .len() gives the length of the vec.
        // It must be at least 5.
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

#[allow(unused_variables)]
fn main() {
    println!("Option and Result");

    let new_vec = vec![1, 2];
    // let index = take_fifth_without_option(new_vec);
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!(
        "{:?}, {:?}",
        take_fifth_with_option(new_vec),
        take_fifth_with_option(bigger_vec)
    );

    /*
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!(
        "{:?}, {:?}",
        take_fifth_with_option(new_vec).unwrap(),
        take_fifth_with_option(bigger_vec).unwrap()
    ); */

    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new(); // Make a new vec to hold our options
                                     // The vec is type: Vec<Option<i32>>, That means a vec of Option<i32>.

    option_vec.push(take_fifth_with_option(new_vec)); // This pushes "None" into the vec
    option_vec.push(take_fifth_with_option(bigger_vec)); // This pushes "Some(5)" into the vec

    handle_option(option_vec); // handle_option looks at every option in the vec.
                               // it prints the value if it is Some. It doesn't touch it if it is None.

    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth_with_option(vec);
        if inside_number.is_some() {
            //    .is_some() returns true if we get Some, false if we get None
            println!("We got: {}", inside_number.unwrap()); // now it is safe to use .unwrap() because we already checked
        } else {
            println!("We got nothing.");
        }
    }

    /* Result */

    if give_result(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }

    let mut result_vec = Vec::new(); // Create a new vec for the results
    for number in 2..7 {
        result_vec.push(check_if_five(number)); // push each result into the vec
    }
    println!("{:?}", result_vec);

    /*     let error_value: Result<i32, &str> = Err("There was an error"); // Create a Result that is already an Err
    println!("{}", error_value.unwrap()); // Unwrap it */

    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0); // 0 to get the first number
    let get_two = my_vec.get(10); // Returns None
    println!("{:?}", get_one);
    println!("{:?}", get_two);

    let my_vec = vec![2, 3, 4];
    for index in 0..10 {
        match my_vec.get(index) {
            Some(number) => println!("The number is: {}", number),
            None => {}
        }
    }

    let my_vec = vec![2, 3, 4];
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is {}", number);
        }
    }

    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}: ", city[0]); // In our data, every first item is the city name
        while let Some(information) = city.pop() {
            // This means: keep going until you can't pop anymore
            // When the vector reaches 0 items, it will return None
            // and it will stop.
            if let Ok(number) = information.parse::<i32>() {
                // Try to parse the variable we called information
                // This returns a result. If it's Ok(number), it will print it
                println!("The number is: {}", number);
            }
            // We don't write anything here because we do nothing if we get an error.
        }
    }
}

/* Result */
fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(());
    } else {
        return Err(());
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()), // This is our error message
    }
}

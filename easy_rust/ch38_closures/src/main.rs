use std::collections::HashMap;

#[allow(dead_code)]
struct Company {
    name: String,
    ceo: Option<String>,
}
impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        }; // ceo is decided, so now we return Self
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone() // Just returns a clone of the CEO (struct is not Copy)
    }
}

fn main() {
    println!("Closures");

    let my_closure = || println!("This is a closure");
    my_closure();
    let my_closure = |x: i32| println!("{}", x);
    my_closure(5);
    my_closure(5 + 5);
    let my_closure = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {} and {}.", number, other_number);
        // This closure can be as long as we want, just like a function.
    };
    my_closure();

    let number_one = 6;
    let number_two = 10;
    let my_closure = || println!("{}", number_one + number_two);
    my_closure();

    let number_one = 6;
    let number_two = 10;
    let my_closure = |x: i32| println!("{}", number_one + number_two + x);
    my_closure(5);

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        // try to unwrap. If it doesn't work,
        // see if my_vec has something at index [0]
        // Give the number at index 0 if there is something
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0 // otherwise give a &0
        }
    });
    println!("{}", fourth);

    let num_vec = vec![2, 4, 6];
    let double_vec = num_vec // take num_vec
        .iter() // iterate over it
        .map(|number| number * 2) // for each item, multiply by two
        .collect::<Vec<i32>>(); // then make a new Vec from this
    println!("{:?}", double_vec);

    let num_vec = vec![10, 9, 8];
    num_vec
        .iter() // iterate over num_vec
        .enumerate() // get (index, number)
        .for_each(|(index, number)| println!("Index number {} has number {}", index, number));

    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>
    let number_word_hashmap = some_numbers
        .into_iter() // now it is an iter
        .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together
        .collect::<HashMap<_, _>>();
    println!(
        "For key {} we get {}.",
        2,
        number_word_hashmap.get(&2).unwrap()
    );

    let numbers_together = "140399923481800622623218009598281";
    for (index, number) in numbers_together.char_indices() {
        match (index % 3, number) {
            (0..=1, number) => print!("{}", number), // just print the number if there is a remainder
            _ => print!("{}\t", number),             // otherwise print the number with a tab space
        }
    }

    let my_vec = vec![8, 9, 10];
    println!(
        "{:?}",
        my_vec
            .iter()
            .for_each(|_| println!("We didn't use the variables at all"))
    );

    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let filtered_months = months
        .into_iter() // make an iter
        .filter(|month| month.len() < 5) // We don't want months more than 5 bytes in length
        .filter(|month| month.contains("u")) // We know that each letter is one byte so .len() is fine
        .collect::<Vec<&str>>(); // Also we only like months with the letter u
    println!("{:?}", filtered_months);

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let all_the_ceos = company_vec
        .into_iter()
        .filter_map(|company| company.get_ceo()) // filter_map needs Option<T>
        .collect::<Vec<String>>();
    println!("{:?}", all_the_ceos);

    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let actual_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
    println!("{:?}", actual_numbers);

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let mut results_vec = vec![]; // Pretend we need to gather error results too
    company_vec
        .iter()
        .for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));
    for item in results_vec {
        println!("{:?}", item);
    }

    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];
    let mut results_vec = vec![]; // Pretend we need to gather error results too
    company_vec.iter().for_each(|company| {
        results_vec.push(company.get_ceo().ok_or_else(|| {
            let err_message = format!("No CEO found for {}", company.name);
            err_message
        }));
    });
    for item in results_vec {
        println!("{:?}", item);
    }

    let new_vec = vec![8, 9, 0]; // just a vec with numbers
    let number_to_add = 5; // use this in the math later
    let mut empty_vec = vec![]; // results go in here
    for index in 0..5 {
        empty_vec.push(
            new_vec
                .get(index)
                .and_then(|number| Some(number + 1))
                .and_then(|number| Some(number + number_to_add)),
        )
    }
    println!("{:?}", empty_vec);

    let first_try = vec![
        Some("success!"),
        None,
        Some("success!"),
        Some("success!"),
        None,
    ];
    let second_try = vec![
        None,
        Some("success!"),
        Some("success!"),
        Some("success!"),
        Some("success!"),
    ];
    let third_try = vec![
        Some("success!"),
        Some("success!"),
        Some("success!"),
        Some("success!"),
        None,
    ];

    for i in 0..first_try.len() {
        println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
    }
}

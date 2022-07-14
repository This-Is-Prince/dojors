// use std::collections::HashMap;

// #[allow(dead_code)]
// struct Company {
//     name: String,
//     ceo: Option<String>,
// }
// impl Company {
//     fn new(name: &str, ceo: &str) -> Self {
//         let ceo = match ceo {
//             "" => None,
//             ceo => Some(ceo.to_string()),
//         }; // ceo is decided, so now we return Self
//         Self {
//             name: name.to_string(),
//             ceo,
//         }
//     }

//     fn get_ceo(&self) -> Option<String> {
//         self.ceo.clone() // Just returns a clone of the CEO (struct is not Copy)
//     }
// }

// fn main() {
//     println!("Closures");

//     let my_closure = || println!("This is a closure");
//     my_closure();
//     let my_closure = |x: i32| println!("{}", x);
//     my_closure(5);
//     my_closure(5 + 5);
//     let my_closure = || {
//         let number = 7;
//         let other_number = 10;
//         println!("The two numbers are {} and {}.", number, other_number);
//         // This closure can be as long as we want, just like a function.
//     };
//     my_closure();

//     let number_one = 6;
//     let number_two = 10;
//     let my_closure = || println!("{}", number_one + number_two);
//     my_closure();

//     let number_one = 6;
//     let number_two = 10;
//     let my_closure = |x: i32| println!("{}", number_one + number_two + x);
//     my_closure(5);

//     let my_vec = vec![8, 9, 10];
//     let fourth = my_vec.get(3).unwrap_or_else(|| {
//         // try to unwrap. If it doesn't work,
//         // see if my_vec has something at index [0]
//         // Give the number at index 0 if there is something
//         if my_vec.get(0).is_some() {
//             &my_vec[0]
//         } else {
//             &0 // otherwise give a &0
//         }
//     });
//     println!("{}", fourth);

//     let num_vec = vec![2, 4, 6];
//     let double_vec = num_vec // take num_vec
//         .iter() // iterate over it
//         .map(|number| number * 2) // for each item, multiply by two
//         .collect::<Vec<i32>>(); // then make a new Vec from this
//     println!("{:?}", double_vec);

//     let num_vec = vec![10, 9, 8];
//     num_vec
//         .iter() // iterate over num_vec
//         .enumerate() // get (index, number)
//         .for_each(|(index, number)| println!("Index number {} has number {}", index, number));

//     let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
//     let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>
//     let number_word_hashmap = some_numbers
//         .into_iter() // now it is an iter
//         .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together
//         .collect::<HashMap<_, _>>();
//     println!(
//         "For key {} we get {}.",
//         2,
//         number_word_hashmap.get(&2).unwrap()
//     );

//     let numbers_together = "140399923481800622623218009598281";
//     for (index, number) in numbers_together.char_indices() {
//         match (index % 3, number) {
//             (0..=1, number) => print!("{}", number), // just print the number if there is a remainder
//             _ => print!("{}\t", number),             // otherwise print the number with a tab space
//         }
//     }

//     let my_vec = vec![8, 9, 10];
//     println!(
//         "{:?}",
//         my_vec
//             .iter()
//             .for_each(|_| println!("We didn't use the variables at all"))
//     );

//     let months = vec![
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//     let filtered_months = months
//         .into_iter() // make an iter
//         .filter(|month| month.len() < 5) // We don't want months more than 5 bytes in length
//         .filter(|month| month.contains("u")) // We know that each letter is one byte so .len() is fine
//         .collect::<Vec<&str>>(); // Also we only like months with the letter u
//     println!("{:?}", filtered_months);

//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unknown"),
//         Company::new("Ovintiv", "Doug Suttles"),
//         Company::new("The Red-Headed League", ""),
//         Company::new("Stark Enterprises", ""),
//     ];
//     let all_the_ceos = company_vec
//         .into_iter()
//         .filter_map(|company| company.get_ceo()) // filter_map needs Option<T>
//         .collect::<Vec<String>>();
//     println!("{:?}", all_the_ceos);

//     let user_input = vec![
//         "8.9",
//         "Nine point nine five",
//         "8.0",
//         "7.6",
//         "eleventy-twelve",
//     ];
//     let actual_numbers = user_input
//         .into_iter()
//         .filter_map(|input| input.parse::<f32>().ok())
//         .collect::<Vec<f32>>();
//     println!("{:?}", actual_numbers);

//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unknown"),
//         Company::new("Ovintiv", "Doug Suttles"),
//         Company::new("The Red-Headed League", ""),
//         Company::new("Stark Enterprises", ""),
//     ];
//     let mut results_vec = vec![]; // Pretend we need to gather error results too
//     company_vec
//         .iter()
//         .for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));
//     for item in results_vec {
//         println!("{:?}", item);
//     }

//     let company_vec = vec![
//         Company::new("Umbrella Corporation", "Unknown"),
//         Company::new("Ovintiv", "Doug Suttles"),
//         Company::new("The Red-Headed League", ""),
//         Company::new("Stark Enterprises", ""),
//     ];
//     let mut results_vec = vec![]; // Pretend we need to gather error results too
//     company_vec.iter().for_each(|company| {
//         results_vec.push(company.get_ceo().ok_or_else(|| {
//             let err_message = format!("No CEO found for {}", company.name);
//             err_message
//         }));
//     });
//     for item in results_vec {
//         println!("{:?}", item);
//     }

//     let new_vec = vec![8, 9, 0]; // just a vec with numbers
//     let number_to_add = 5; // use this in the math later
//     let mut empty_vec = vec![]; // results go in here
//     for index in 0..5 {
//         empty_vec.push(
//             new_vec
//                 .get(index)
//                 .and_then(|number| Some(number + 1))
//                 .and_then(|number| Some(number + number_to_add)),
//         )
//     }
//     println!("{:?}", empty_vec);

//     let first_try = vec![
//         Some("success! 1"),
//         None,
//         Some("success! 1"),
//         Some("success! 1"),
//         None,
//     ];
//     let second_try = vec![
//         None,
//         Some("success! 2"),
//         Some("success! 2"),
//         Some("success! 2"),
//         Some("success! 2"),
//     ];
//     let third_try = vec![
//         Some("success! 3"),
//         Some("success! 3"),
//         Some("success! 3"),
//         Some("success! 3"),
//         None,
//     ];

//     for i in 0..first_try.len() {
//         println!("{:?}", first_try[i].and(second_try[i]).and(third_try[i]));
//     }

//     let char_vec = ('a'..'働').collect::<Vec<char>>();
//     in_char_vec(&char_vec, 'i');
//     in_char_vec(&char_vec, '뷁');
//     in_char_vec(&char_vec, '鑿');

//     let smaller_vec = ('A'..'z').collect::<Vec<char>>();
//     println!(
//         "All alphabetic? {}",
//         smaller_vec.iter().all(|&x| x.is_alphabetic())
//     );
//     println!(
//         "All less than the character 행? {}",
//         smaller_vec.iter().all(|&x| x < '행')
//     );

//     let mut big_vec = vec![6; 1000];
//     big_vec.push(5);

//     let mut iterator = big_vec.iter().rev();
//     println!("{:?}", iterator.next());
//     println!("{:?}", iterator.next());

//     let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
//     println!("{:?}", num_vec.iter().find(|&number| number % 3 == 0)); // find takes a reference, so we give it &number
//     println!("{:?}", new_vec.iter().find(|&number| number * 2 == 30));

//     println!("{:?}", num_vec.iter().position(|&number| number % 3 == 0));
//     println!("{:?}", new_vec.iter().position(|&number| number * 2 == 30));

//     let even_odd = vec!["even", "odd"];
//     let even_odd_vec = (0..6)
//         .zip(even_odd.into_iter().cycle())
//         .collect::<Vec<(_, _)>>();
//     println!("{:?}", even_odd_vec);

//     let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
//     let skip_then_ten_chars = ('a'..).skip(1300).take(10).collect::<Vec<char>>();
//     println!("{:?}", ten_chars);
//     println!("{:?}", skip_then_ten_chars);

//     let some_numbers = vec![1, 2, 3, 4, 5];
//     println!(
//         "{}",
//         some_numbers
//             .iter()
//             .fold(1, |total_so_far, next_number| total_so_far * next_number)
//     );

//     let a_string = "I don't have any dashes in me.";
//     println!(
//         "{}",
//         a_string
//             .chars()
//             .fold("-".to_string(), |mut string_so_far, next_char| {
//                 // Start with a String "-". Bring it in as mutable each time along with the next char
//                 string_so_far.push(next_char); // Push the char on, then '-'
//                 string_so_far.push('-');
//                 string_so_far // Don't forget to pass it on to the next loop
//             })
//     );

//     let num_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
//     for chunk in num_vec.chunks(3) {
//         println!("{:?}", chunk);
//     }
//     for window in num_vec.windows(3) {
//         println!("{:?}", window);
//     }

//     let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
//     let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>(); // This is Vec<usize, &str> but we just tell Rust to do it
//     println!("{:?}", rule_locations);

//     let just_numbers = vec![1, 5, 100];
//     let mut number_iter = just_numbers.iter().peekable(); // This actually creates a type of iterator called Peekable

//     for _ in 0..3 {
//         println!("I love the number {}", number_iter.peek().unwrap());
//         println!("I really love the number {}", number_iter.peek().unwrap());
//         println!("{} is such a nice number", number_iter.peek().unwrap());
//         number_iter.next();
//     }

//     let locations = vec![
//         ("Nevis", 25),
//         ("Taber", 8428),
//         ("Markerville", 45),
//         ("Cardston", 3585),
//     ];
//     let mut location_iter = locations.iter().peekable();
//     while location_iter.peek().is_some() {
//         match location_iter.peek() {
//             Some((name, number)) if *number < 100 => {
//                 println!("Found a hamlet: {} with {} people", name, number);
//             }
//             Some((name, number)) => println!("Found a town: {} with {} people", name, number),
//             None => break,
//         }
//         location_iter.next();
//     }

//     let vec_of_names = vec![
//         "Caesar",
//         "Frodo Baggins",
//         "Bilbo Baggins",
//         "Jean-Luc Picard",
//         "Data",
//         "Rand Al'Thor",
//         "Paul Atreides",
//         "Barack Hussein Obama",
//         "Bill Jefferson Clinton",
//     ];
//     let mut iter_of_names = vec_of_names.iter().peekable();
//     let mut all_names = Names {
//         // start an empty Names struct
//         one_word: vec![],
//         two_words: vec![],
//         three_words: vec![],
//     };

//     while iter_of_names.peek().is_some() {
//         let next_item = iter_of_names.next().unwrap(); // We can use .unwrap() because we it is Some
//         match next_item.match_indices(' ').collect::<Vec<_>>().len() {
//             // Create a quick vec using .match_indices and check the length
//             0 => all_names.one_word.push(next_item.to_string()),
//             1 => all_names.two_words.push(next_item.to_string()),
//             _ => all_names.three_words.push(next_item.to_string()),
//         }
//     }
//     println!("{:?}", all_names);
// }

// fn in_char_vec(char_vec: &Vec<char>, check: char) {
//     println!(
//         "Is {} inside? {}",
//         check,
//         char_vec.iter().any(|&character| character == check)
//     );
// }

// #[derive(Debug)]
// struct Names {
//     one_word: Vec<String>,
//     two_words: Vec<String>,
//     three_words: Vec<String>,
// }

fn main() {
    let my_closure = || println!("This is a closure");
    my_closure();

    let a = 1;
    let sum = |x: i32| x + a;
    println!("{}", sum(2));

    let body = || {
        println!("Body");
        "body"
    };
    println!("{}", body());
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque; // This is so we can just write HashMap instead of std::collections::HashMap every time

/* HashMap (and BTreeMap) */
struct City {
    name: String,
    population: HashMap<u32, u32>, // This will have the year and the population for the year
}

fn hashmap() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(), // So far the HashMap is empty
    };

    tallinn.population.insert(1372, 3_250); // insert three dates
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!(
            "In the year {} the city of {} had a population of {}.",
            year, tallinn.name, population
        );
    }

    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];
    let mut city_hashmap = HashMap::new();
    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }
    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1));

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    if book_hashmap.get(&1).is_none() {
        // is_none() returns a bool: true if it's None, false if it's Some
        book_hashmap.insert(1, "Le Petit Prince");
    }
    println!("{:?}", book_hashmap.get(&1));

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ]; // Eye of the World appears twice
    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_of_false) in book_hashmap {
        println!("Do we have {}? {}", book, true_of_false);
    }

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ]; // Eye of the World appears twice
    let mut book_hashmap = HashMap::new();
    for book in book_collection {
        *(book_hashmap.entry(book).or_insert(0)) += 1;
    }
    for (book, number) in book_hashmap {
        println!("{} {}", book, number);
    }

    let data = vec![
        // This is the raw data
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];
    let mut survey_hash = HashMap::new();
    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1); // This pushes the number into the Vec inside
    }
    for (male_or_female, numbers) in survey_hash {
        println!("{:?}, {:?}", male_or_female, numbers);
    }
}

struct City1 {
    name: String,
    population: BTreeMap<u32, u32>, // This will have the year and the population for the year
}

fn btree_map() {
    let mut tallinn = City1 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(), // So far the BTreeMap is empty
    };

    tallinn.population.insert(1372, 3_250); // insert three dates
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!(
            "In the year {} the city of {} had a population of {}.",
            year, tallinn.name, population
        );
    }
}

fn hashset() {
    println!("HashSet");

    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11,
    ];

    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }
    let hashset_length = number_hashset.len(); // The length tells us how many numbers are in it
    println!(
        "There are {} unique numbers, so we are missing {}.",
        hashset_length,
        100 - hashset_length
    );

    // Let's see what numbers we are missing
    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            // If .get() returns None,
            missing_vec.push(number);
        }
    }
    print!("It does not contain: ");
    for number in missing_vec {
        print!("{} ", number);
    }

    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11,
    ];
    let mut number_btree_set = BTreeSet::new();
    for number in many_numbers {
        number_btree_set.insert(number);
    }
    for entry in number_btree_set {
        print!("{}", entry);
    }
}

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number);
    }
    remainder_vec
}

fn binary_heap() {
    println!("Binary Heap");

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30]; // These numbers are in order
    let mut my_heap = BinaryHeap::new();
    for number in many_numbers {
        my_heap.push(number);
    }
    while let Some(number) = my_heap.pop() {
        // .pop() returns Some(number) if a number is there, None if not. It pops from the front
        println!(
            "Popped off {}. Remaining numbers are: {:?}",
            number,
            show_remainder(&my_heap)
        );
    }

    let mut jobs = BinaryHeap::new();

    // Add jobs to do throughout the day
    jobs.push((100, "Write back to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    while let Some(job) = jobs.pop() {
        println!("You need to: {}", job.1);
    }
}

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    // Each item is a (&str, bool)
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap(); // pop off the back
                                                   // now it's done - mark as true
                                                   // put it at the front now
    task_done.1 = true;
    input.push_front(task_done);
}

fn vec_deque() {
    /*
    let mut my_vec = vec![0; 600_000];
    for _ in 0..600_000 {
        my_vec.remove(0);
    }
    */

    /*
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    for _ in 0..600_000 {
        my_vec.remove(0);
    }
    */

    let mut my_vecdeque = VecDeque::new();
    let things_to_do = vec![
        "send email to customer",
        "add new product to list",
        "phone Loki back",
    ];
    for thing in things_to_do {
        my_vecdeque.push_front((thing, false));
    }
    done(&mut my_vecdeque);
    done(&mut my_vecdeque);

    check_remaining(&my_vecdeque);
    for task in my_vecdeque {
        print!("{:?}", task);
    }
}

fn main() {
    println!("Other Collections");

    hashmap();
    btree_map();
    hashset();
    binary_heap();
    vec_deque();
}

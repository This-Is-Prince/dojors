/*
================
HashMap and BTreeMap
================
*/
/* #[allow(unused)]
use std::collections::{BTreeMap, HashMap};

#[allow(unused)]
struct City {
    name: String,
    population: HashMap<u32, u32>,
    sorted_population: BTreeMap<u32, u32>,
}

fn first(is_run: bool) {
    if is_run {
        let mut tallinn = City {
            name: "Tallinn".to_string(),
            population: HashMap::new(),
            sorted_population: BTreeMap::new(),
        };
        tallinn.population.insert(40, 400);
        tallinn.population.insert(50, 200);
        tallinn.population.insert(60, 500);
        tallinn.population.insert(20, 5000);

        tallinn.sorted_population.insert(40, 400);
        tallinn.sorted_population.insert(50, 200);
        tallinn.sorted_population.insert(60, 500);
        tallinn.sorted_population.insert(20, 5000);

        for (key, value) in tallinn.population.clone() {
            println!("{}:{}", key, value);
        }
        for (key, value) in tallinn.sorted_population.clone() {
            println!("{}:{}", key, value);
        }
        println!("{:?}", tallinn.population);
        println!("{:?}", tallinn.sorted_population);
    }
}

fn second(is_run: bool) {
    if is_run {
        let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
        let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

        let mut city_hashmap = HashMap::new();

        for city in canadian_cities {
            city_hashmap.insert(city, "Canadian");
        }
        for city in german_cities {
            city_hashmap.insert(city, "Canadian");
        }
        println!("{:?}", city_hashmap["Bielefeld"]);
        println!("{:?}", city_hashmap.get("Bielefeld"));
        println!("{:?}", city_hashmap.get("Bielefeldd"));
        println!("{:?}", city_hashmap);
        if let Some(city) = city_hashmap.get("Bielefeld") {
            println!("{}", city);
        }
    }
}

fn third(is_run: bool) {
    if is_run {
        let mut book_hashmap = HashMap::new();
        book_hashmap.insert(1, "The Witcher");
        book_hashmap.insert(1, "A Song Of Ice And Fire");
        book_hashmap.insert(1, "Wheel Of The Time");
        book_hashmap.insert(1, "Lords Of The Rings");
        println!("{:?}", book_hashmap);
    }
}

fn fourth(is_run: bool) {
    if is_run {
        let mut book_hashmap = HashMap::new();
        book_hashmap.insert(1, "The Witcher");
        if book_hashmap.get(&1).is_none() {
            book_hashmap.insert(1, "Lords Of The Rings");
        }
        println!("{:?}", book_hashmap.get(&1));
    }
}

fn five(is_run: bool) {
    if is_run {
        let books = vec![
            "The Witcher",
            "Lords Of The Rings",
            "A Song Of Ice And Fire",
            "Wheel Of The Time",
            "Lords Of The Rings",
            "A Song Of Ice And Fire",
            "Wheel Of The Time",
            "Lords Of The Rings",
        ];
        let mut book_hashmap = HashMap::new();
        for book in books {
            book_hashmap.entry(book).or_insert(true);
        }
        for (book, true_or_false) in book_hashmap {
            println!("Do we have {}? {}", book, true_or_false);
        }
    }
}

fn six(is_run: bool) {
    if is_run {
        let books = vec![
            "The Witcher",
            "Lords Of The Rings",
            "A Song Of Ice And Fire",
            "Wheel Of The Time",
            "Lords Of The Rings",
            "A Song Of Ice And Fire",
            "Wheel Of The Time",
            "Lords Of The Rings",
        ];
        let mut book_hashmap = HashMap::new();
        for book in books {
            *book_hashmap.entry(book).or_insert(0) += 1;
        }
        for (book, no) in book_hashmap {
            println!("{} book has {} many copies", book, no);
        }
    }
}

fn seven(is_run: bool) {
    if is_run {
        let books = vec![
            "The Witcher",
            "Lords Of The Rings",
            "A Song Of Ice And Fire",
            "Wheel Of The Time",
            "Lords Of The Rings",
            "A Song Of Ice And Fire",
            "Wheel Of The Time",
            "Lords Of The Rings",
        ];
        let mut book_hashmap = HashMap::new();
        let mut book_hashmap2 = HashMap::new();
        for book in books {
            if let Some(mut v) = book_hashmap2.insert(book, 0) {
                v = 10;
                println!("{}", v);
            }
            *book_hashmap.entry(book).or_insert(0) += 1;
        }
        for (book, no) in book_hashmap {
            println!("{} book has {} many copies", book, no);
        }
        println!("{:?}", book_hashmap2);
    }
}

fn eight(is_run: bool) {
    if is_run {
        let data = vec![
            ("male", 9),
            ("female", 5),
            ("male", 0),
            ("female", 6),
            ("male", 10),
            ("female", 10),
        ];
        let mut survey_hash = HashMap::new();
        for item in data {
            (survey_hash.entry(item.0).or_insert(Vec::new())).push(item.1);
        }
        println!("{:?}", survey_hash);
    }
}

#[allow(unused)]
fn main() {
    first(false);
    second(false);
    third(false);
    fourth(false);
    five(false);
    six(false);
    seven(false);
    eight(true);
}
 */

/*
================
HashSet and BTreeSet
================
*/
/* #[allow(unused)]
use std::collections::{BTreeSet, HashSet};

fn first(is_run: bool) {
    if is_run {
        let many_numbers = vec![
            94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81,
            66, 51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10,
            35, 20, 35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95,
            13, 60, 59, 96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80,
            56, 22, 67, 82, 58, 64, 80, 16, 61, 57, 14, 11,
        ];

        let mut number_hashset = HashSet::new();

        for number in many_numbers {
            number_hashset.insert(number);
        }
        for number in number_hashset.clone() {
            print!("{} ", number);
        }
        let hashset_length = number_hashset.len();
        println!(
            "There are {} unique numbers out of {}, so we are missing {}.",
            hashset_length,
            100,
            100 - hashset_length
        );

        let mut missing_vec = vec![];
        for number in 0..100 {
            if number_hashset.get(&number).is_none() {
                missing_vec.push(number);
            }
        }
        print!("It does not contain: ");
        for number in missing_vec {
            print!("{} ", number);
        }
    }
}
fn second(is_run: bool) {
    if is_run {
        let many_numbers = vec![
            94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81,
            66, 51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10,
            35, 20, 35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95,
            13, 60, 59, 96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80,
            56, 22, 67, 82, 58, 64, 80, 16, 61, 57, 14, 11,
        ];
        let mut number_btreeset = BTreeSet::new();
        for number in many_numbers {
            number_btreeset.insert(number);
        }
        for number in number_btreeset {
            print!("{} ", number);
        }
    }
}

fn main() {
    first(false);
    second(true);
}
 */

/*
================
BinaryHeap
================
*/
/* #[allow(unused)]
use std::collections::BinaryHeap;

#[allow(unused)]
fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number);
    }
    remainder_vec
}
fn first(is_run: bool) {
    if is_run {
        let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
        let mut my_heap = BinaryHeap::new();
        for number in many_numbers {
            my_heap.push(number);
        }
        while let Some(number) = my_heap.pop() {
            println!(
                "Popped off {}. Remaining numbers are: {:?}",
                number,
                show_remainder(&my_heap)
            );
        }
    }
}

fn second(is_run: bool) {
    if is_run {
        let mut jobs = BinaryHeap::new();
        jobs.push((100, "Write back to email from the CEO"));
        jobs.push((80, "Finish the report today"));
        jobs.push((5, "Watch some YouTube"));
        jobs.push((70, "Tell your team members thanks for always working hard"));
        jobs.push((30, "Plan who to hire next for the team"));

        while let Some(job) = jobs.pop() {
            println!("You need to: {}", job.1);
        }
    }
}

#[allow(unused)]
fn main() {
    first(false);
    second(true);
}
 */

/*
================
VecDeque
================
*/
#[allow(unused)]
use std::collections::VecDeque;

fn first(is_run: bool) {
    if is_run {
        println!("Start");
        let mut my_vec = vec![0; 600_000];
        for _ in 0..600_000 {
            my_vec.remove(0);
        }
        println!("End");
    }
}

fn second(is_run: bool) {
    if is_run {
        println!("Start Deque");
        let mut my_vec = VecDeque::from(vec![0; 600_000]);
        for _ in 0..600_000 {
            my_vec.pop_front();
        }
        println!("End Deque");
    }
}

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap();
    task_done.1 = true;
    input.push_front(task_done);
}

fn third(is_run: bool) {
    if is_run {
        let mut my_vecdeque = VecDeque::new();
        let things_to_do = vec![
            "send email to customer",
            "add new product to list",
            "phone loki back",
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
}

fn main() {
    first(false);
    second(false);
    third(true);
}

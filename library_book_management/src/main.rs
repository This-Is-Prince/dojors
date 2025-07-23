mod book;
mod member;
mod loan;
mod library;

use std::collections::HashMap;

const LIBRARY_NAME: &str = "City Central Library";
const MAX_BOOKS_PER_MEMBER: u32 = 5;
const LATE_FEE_PER_DAY: f64 = 0.50;
const LIBRARY_OPENS: u32 = 9;
const LIBRARY_CLOSES: u32 = 21;

fn demonstrate_variables() {
    println!("\n=== Demonstrating Variables and Mutability ===\n");

    let book_count = 100;
    println!("Book count (immutable): {}", book_count);

    let mut available_books = 100;
    println!("Available books before: {}", available_books);
    available_books = available_books - 1;
    println!("Available books after: {}", available_books);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2: {}", s2);
}

fn demonstrate_data_types() {
    println!("\n === Demonstrating Data Types ===\n");

    let small_number: i8 = -128;
    let medium_number: i32 = 42;
    let big_number: i64 = 9_223_372_036_854_775_807;
    let unsigned_number: u32 = 300;

    let price: f32 = 19.99;
    let precise_price: f64 = 19.9999999;

    let is_open: bool = true;
    let is_closed: bool = false;

    let letter: char = 'A';
    let emoji: char = '📚';

    let book_title: &str = "The Rust Programming Language";

    let mut author = String::from("Steve Klabnik");
    author.push_str(" and Carol Nichols");
    println!("Book: {} by {}", book_title, author);
    
    let book_info: (i32, &str, f64) = (12345, "Programming", 29.99);
    println!("Book ID: {}, Category: {}, Price: ${}", book_info.0, book_info.1, book_info.2);

    let (id, category, price) = book_info;
    println!("Destructured - ID: {}, Category: {}, Price: ${}", id, category, price);

    let empty_tuple: () = ();
    let single_element: (i32, ) = (42, );
    let pair: (String, i32) = (String::from("Harry Potter"), 7);
    let triple: (char, bool, f64) = ('A', true, 3.14);

    let nested: ((i32, i32), (f64, f64)) = ((1, 2), (3.0, 4.0));
    let ((x1, y1), (x2, y2)) = nested;

    let weekdays: [&str; 5] = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let all_zeros: [i32; 10] = [0; 10];

    let first_day = weekdays[0];
    let last_day = weekdays[4];

    println!("Library open {} to {}", first_day, last_day);
}

fn print_library_header() {
    println!("\n╔══════════════════════════════╗");
    println!("║    {}    ║", LIBRARY_NAME);
    println!("╚══════════════════════════════╝");
}

fn calculate_fine(days_late: u32, is_student: bool) -> f64 {
    let base_fine = days_late as f64 * LATE_FEE_PER_DAY;

    if is_student {
        return base_fine * 0.5;
    }

    base_fine
}

fn extend_due_date(days: &mut u32, extension: u32) {
    *days += extension;
}

fn get_book_location(book_id: u32) -> (char, u32, u32) {
    let section = ((book_id / 1000) as u8 + b'A') as char;
    let shelf = (book_id % 1000) / 10;
    let position = book_id % 10;

    (section, shelf, position)
}

fn is_library_open(hour: u32) -> bool {
    hour >= LIBRARY_OPENS && hour < LIBRARY_CLOSES
}

fn library_panic(msg: &str) -> ! {
    panic!("LIBRARY SYSTEM ERROR :{}", msg);
}

fn demonstrate_control_flow() {
    println!("\n=== Demonstrating  Control Flow ===\n");

    let current_hour = 14;

    if is_library_open(current_hour) {
        println!("Library is open!");
    } else {
        println!("Library is closed!");
    }

    let status = if is_library_open(current_hour) {
        "Open for business"
    } else {
        "Closed"
    };
    println!("Status: {}", status);
    

    let member_age = 65;
    let membership_fee = if member_age < 18 {
        0.0
    } else if member_age >= 65 {
        10.0
    } else {
        25.0
    };
    println!("Membership fee: ${}", membership_fee);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    let mut books_to_process = 5;
    while books_to_process > 0 {
        println!("Processing book {} ...", books_to_process);
        books_to_process -= 1;
    }

    println!("\nLibrary sections");
    for section in 'A'..='E' {
        println!("Section {}", section);
    }

    let popular_genres: [&str; 4] = ["Fiction", "Non-fiction", "Science", "History"];
    for (index, genre) in popular_genres.iter().enumerate() {
        println!("{}. {}", index + 1, genre);
    }

    'outer: for shelf in 1..=3 {
        'inner: for book in 1..=5 {
            if shelf == 2 && book == 3 {
                println!("Found the book at shelf {}, position {}!", shelf, book);
                break 'outer;
            }
        }
    }

    let book_code = 'B';
    let section_name = match book_code {
        'A' => "Arts",
        'B' => "Biography",
        'C' => "Computing",
        'F' => "Fiction",
        'H' => "History",
        'S' => "Science",
        _ => "General",
    };
    println!("\n Book code {} means section: {}", book_code, section_name);

    let book_count: u32 = 15;
    match book_count {
        0 => println!("No books"),
        1 => println!("One book"),
        2..=5 => println!("A few books"),
        6..=20 => println!("Several books"),
        n if n > 100 => println!("That's a lot of books!"),
        _ => println!("Many books"),
    }

    let day = "Saturday";
    match day  {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => {
            println!("Library hours: 9 AM - 9 PM");
        }
        "Saturday" | "Sunday" => {
            println!("Library hours: 10 AM - 6 PM");
        }
        _ => println!("Invalid day"),
    }
}


fn main() {
    println!("Hello, world!");
}

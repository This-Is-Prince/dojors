pub fn run_all() {
    println!("\n--- Ownership demos ---");

    let a = String::from("move me");
    let b = a;
    println!("b has: {b}");

    let x = 5;
    let y = x;
    println!("x {x}, y {y}");

    let s1 = String::from("hello");
    print_len(&s1);
    println!("Still own s1: {s1}");

    let mut s2 = String::from("hola");
    add_txt(&mut s2);
    println!("Changed s2: {s2}");

    let word = first_word(&s2);
    println!("First word slice: {word}");
}

fn print_len(s: &String) {
    println!("len = {}", s.len());
}

fn add_txt(s: &mut String) {
    s.push_str(" amigo");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
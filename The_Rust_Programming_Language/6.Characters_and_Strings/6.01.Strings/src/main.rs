#[allow(unused_variables)]
fn strings() {
    // utf-8
    let s: &'static str = "hello there!"; // &str = string slice
                                          // let h=s[0]; // Error

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is: {}", first_char);
    }

    // Heap
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("letters is: {}", letters);

    // &str <> String
    let u: &str = &letters;
    println!("{}", u);

    // concatenation must be in the form
    // String + &str
    // let concatenation = letters + "letters";
    // let concatenation = letters + s;
    // let concatenation = letters + &String::from("letters");

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn main() {
    strings();
}

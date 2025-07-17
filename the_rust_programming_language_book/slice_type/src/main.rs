fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("{word}");

    let s1: String = String::from("hello world");

    let hello: &str = &s1[0..5];
    let world: &str = &s1[6..11];
    let ld: &str = &s1[9..=10];

    println!("{hello} {world} {ld}");

    let word1: &str = first_word1(&s1);
    println!("{word1}")
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn first_word1(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
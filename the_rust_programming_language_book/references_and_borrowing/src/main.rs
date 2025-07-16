fn main() {
    let mut s1: String = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of `{s1}` is {len}.");
    
    change(&mut s1);

    let len = calculate_length(&s1);
    println!("The length of `{s1}` is {len}.");

    let mut s = String::from("hello");

    let r1 = &s;
    let r2: &String = &s;
    println!("{r1} and {r2}");

    let r3: &mut String = &mut s;
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// fn dangle() -> &String {
//     let s: String = String::from("HI");

//     &s
// }
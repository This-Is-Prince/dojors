fn prints_str(my_str: &str) {
    // it can use &String like a &str
    println!("{}", my_str);
}

fn main() {
    println!("Types of &str");

    let my_string = String::from("I am a string");
    prints_str(&my_string); // we give prints_str a &String
}

#[allow(unused_variables)]
fn main() {
    println!("Tuples");

    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {:?}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth item: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5
    );

    let str_vec = vec!["one", "two", "three"];
    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]); // call them a, b, and c
    println!("{:?}", b);

    let str_vec = vec!["one", "two", "three"];
    let (_, _, variables) = (str_vec[0], str_vec[1], str_vec[2]); // Now it only creates a variable called variable but doesn't make a variable for the others.
    println!("{:?}", variables);
}

#[allow(dead_code)]
fn do_something1() {}

#[allow(dead_code)]
fn do_something2() -> () {}

#[allow(dead_code)]
fn just_prints() {
    println!("I am printing"); // Adding ; means we return an empty tuple
}

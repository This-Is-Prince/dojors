fn tuples() {
    // fn do_something() {}
    // is actually short for:
    // fn do_something() -> () {}

    /*
    fn just_prints() {
        println!("I am printing"); // Adding ; means we return an empty tuple
    }
    */

    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7, false);
    println!(
        "Inside the tuple is: First Item: {:#?} Second Item: {:#?} Third Item: {:#?} Fourth Item: {:#?} Fifth Item: {:#?} Sixth Item: {:#?} Seventh Item: {:#?}",
        random_tuple.0, random_tuple.1,
        random_tuple.2, random_tuple.3,
        random_tuple.4, random_tuple.5,
        random_tuple.6,
    );

    let str_vec = vec!["one", "two", "three"];
    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?} {:?} {:?}", a, b, c);

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{}", variable);
}

fn main() {
    println!("----------Tuples----------");

    tuples();
}

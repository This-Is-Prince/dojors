#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    println!("Vectors");

    let name1 = String::from("Prince");
    let name2 = String::from("Kumar");

    let mut my_vec = Vec::new();
    // If we run the program now, the compiler will give an error.
    // It doesn't know the type of vec.

    my_vec.push(name1); // Now it knows: it's Vec<String>
    my_vec.push(name2);

    let mut my_vec: Vec<String> = Vec::new(); // The compiler knows the type
                                              // so there is no error.

    let mut my_vec = vec![8, 10, 10];

    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Everything is the same as above except we added vec!.
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!(
        "Three to five: {:?},
start at two: {:?},
end at five: {:?}
everything: {:?}",
        three_to_five, start_at_two, end_at_five, everything
    );

    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity()); // 0 elements: prints 0
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // 1 element: prints 4. Vecs with 1 item always start with capacity 4
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // 4 elements: still prints 4.
    num_vec.push('a'); // add one more
    println!("{}", num_vec.capacity()); // prints 8. We have 5 elements, but it doubled 4 to 8 to make space

    println!("With Capacity");
    let mut num_vec = Vec::with_capacity(8); // Give it capacity 8
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one character
    println!("{}", num_vec.capacity()); // prints 8
    num_vec.push('a'); // add one character
    num_vec.push('a'); // add one more // Now we have 5 elements
    println!("{}", num_vec.capacity()); // Still 8

    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into(); // Vec<_> means "choose the Vec type for me"
                                             // Rust will choose Vec<i32>
}

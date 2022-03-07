fn arrays() {
    let array1 = ["One", "Two"]; // This one is type [&str; 2]
    let array2 = ["One", "Two", "Five"]; // But this one is type [&str; 3]. Different type!
    println!("{:#?}", array1);
    println!("{:#?}", array2);

    // let seasons1 = ["Spring", "Summer", "Autumn", "Winter"];
    // let seasons2 = ["Spring", "Summer", "Fall", "Autumn", "Winter"];
    // seasons1.ddd();
    // seasons2.thd();

    let my_array = ["a"; 10];
    println!("{:#?}", my_array);

    let my_numbers = [0, 10, -20];
    println!("{:#?}", my_numbers[1]);

    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    println!("Three to five {three_to_five:#?}, start at two: {start_at_two:#?}, end at five: {end_at_five:#?}, everything: {everything:#?}",three_to_five=three_to_five,start_at_two=start_at_two, end_at_five=end_at_five,everything=everything,);
}

fn main() {
    println!("-------------Collection Types-------------");

    arrays();
}

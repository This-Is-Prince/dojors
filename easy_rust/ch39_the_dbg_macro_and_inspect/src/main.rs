// #[allow(unused_variables)]
// #[allow(unused_assignments)]
// fn main() {
//     println!("The dbg! macro and .inspect");
//     /*
//     let my_number = 8;
//     dbg!(my_number);

//     let mut my_number = dbg!(9);

//     dbg!(my_number += 10);
//     let new_vec = dbg!(vec![8, 9, 10]);
//     let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
//     dbg!(double_vec);
//     */
//     let new_vec = vec![8, 9, 10];

//     /*
//     let double_vec = new_vec
//     .iter()
//     .inspect(|first_item| println!("The item is: {}", first_item))
//     .map(|x| x * 2)
//     .inspect(|next_item| println!("Then it is: {}", next_item))
//     .collect::<Vec<i32>>();
//     */
//     let double_vec = new_vec
//         .iter()
//         .inspect(|first_item| {
//             println!("The item is: {}", first_item);
//             match **first_item % 2 {
//                 // first item is a &&i32 so we use **
//                 0 => println!("It is even."),
//                 _ => println!("It is odd."),
//             }
//             println!("In binary it is {:b}.", first_item);
//         })
//         .map(|x| x * 2)
//         .collect::<Vec<i32>>();
// }

fn main() {
    println!("Hello World!");
    dbg!(println!("Hello World!"));

    let mut x = 1;
    println!("{}", x);
    dbg!(x += 1);
    println!("{}", x);

    let new_vec = vec![1, 2, 3];
    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());
    dbg!(double_vec);

    let new_vec = vec![1, 2, 3];
    let double_vec = new_vec
        .iter()
        .inspect(|first_item| println!("The item is first time: {}", first_item))
        .map(|x| x * 2)
        .inspect(|next_item| println!("The item is second time: {}", next_item))
        .collect::<Vec<i32>>();

    println!("{:?}", double_vec);
}

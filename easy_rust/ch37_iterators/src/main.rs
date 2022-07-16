// #[derive(Debug, Clone)] // we want to print it with {:?}
// &#[allow(dead_code)]
// struct Library {
//     library_type: LibraryType, // this is our enum
//     books: Vec<String>,        // list of books
// }

// #[derive(Debug, Clone)]
// #[allow(dead_code)]
// enum LibraryType {
//     // libraries can be city libraries or country libraries
//     City,
//     Country,
// }

// impl Library {
//     fn add_book(&mut self, book: &str) {
//         // we use add_book to add new books
//         self.books.push(book.to_string()); // we take a &str and turn it into a String, then add it to the Vec
//     }

//     fn new() -> Self {
//         // this creates a new Library
//         Self {
//             library_type: LibraryType::City, // most are in the city so we'll choose City most of the time
//             books: Vec::new(),
//         }
//     }
// }

// impl Iterator for Library {
//     type Item = String;

//     fn next(&mut self) -> Option<String> {
//         match self.books.pop() {
//             Some(book) => Some(book + " is found"), // Rust allows String + &str
//             None => None,
//         }
//     }
// }

// fn main() {
//     println!("Iterators");

//     /*     let vector1 = vec![1, 2, 3]; // we will .iter() and .into_iter() on this one
//     let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
//     let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

//     let mut vector2 = vec![10, 20, 30]; // we will use .iter_mut() on this one
//     vector2.iter_mut().for_each(|x| *x += 10);

//     println!("{:?}", vector1_a);
//     println!("{:?}", vector2);
//     println!("{:?}", vector1_b);

//     let my_vec = vec!['a', 'b', '거', '柳']; // Just a regular Vec
//     let mut my_vec_iter = my_vec.iter(); // This is an Iterator type now, but we haven't called it yet
//     assert_eq!(my_vec_iter.next(), Some(&'a')); // Call the first item with .next()
//     assert_eq!(my_vec_iter.next(), Some(&'b')); // Call the next
//     assert_eq!(my_vec_iter.next(), Some(&'거')); // Again
//     assert_eq!(my_vec_iter.next(), Some(&'柳')); // Again
//     assert_eq!(my_vec_iter.next(), None); // Nothing is Left: just None
//     assert_eq!(my_vec_iter.next(), None); // You can keep calling .next() but it will always be None */
//     let mut my_library = Library::new(); // make a new library
//     my_library.add_book("The Doom of the Darksword"); // add some books
//     my_library.add_book("Demian - die Geschichte einer Jugend");
//     my_library.add_book("구운몽");
//     my_library.add_book("吾輩は猫である");

//     println!("{:?}", my_library.books); // we can print our list of books

//     for item in my_library.clone() {
//         // we can use a for loop now, Give it a clone so Library won't be destroyed
//         println!("{}", item);
//     }
// }

// // an iterator which alternates between Some and None
// struct Alternate {
//     state: i32,
// }
// impl Iterator for Alternate {
//     type Item = i32;

//     fn next(&mut self) -> Option<i32> {
//         let val = self.state;
//         self.state = self.state + 1;

//         // if it's even , Some(i32), else None
//         if val % 2 == 0 {
//             Some(val)
//         } else {
//             None
//         }
//     }
// }

fn main() {
    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b = vector1.into_iter().map(|x| x * 2).collect::<Vec<i32>>();

    let mut vector2 = vec![10, 20, 30];
    println!("{:?}", vector2);
    vector2.iter_mut().for_each(|x| *x = *x * 2);
    println!("{:?}", vector2);
    // println!("{:#?}", vector1); // ERROR, borrow of moved value: `vector1
    println!("{:?}", vector1_a);
    println!("{:?}", vector1_b);
}

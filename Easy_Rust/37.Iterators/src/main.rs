#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}
impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }
    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"),
            None => None,
        }
    }
}

/* ===============2============== */
fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    println!("{:?}", my_library.books);
    for item in my_library {
        println!("{}", item);
    }
    // println!("{:?}", my_library);// Error
}

/* ===============1============== */
/*
fn main() {
    let vector1 = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|x| *x *= 100);
    println!("{:?}", vector1_a);
    println!("{:?}", vector2);
    println!("{:?}", vector1_b);

    let mut v = vec![1, 2, 3, 4];
    for i in v.iter_mut() {
        println!("{}", i);
        *i *= 10;
    }
    println!("{:?}", v);

    let my_vec = vec!['a', 'b', '거', '거'];
    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);
}
*/

use std::fmt;

#[derive(Debug, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub year: u32,
    pub available: bool,
    genre: Genre,
}

#[derive(Debug, Clone)]
enum Genre {
    Fiction,
    NonFiction,
    Science,
    History,
    Biography,
    Other(String),
}

impl Book {
    pub fn new(isbn: String, title: String, author: String, year: u32) -> Self {
        Self {
            isbn,
            title,
            author,
            year,
            available: true,
            genre: Genre::Other("Unclassified".to_string()),
        }
    }

    pub fn display_info(&self) {
        println!("📖 {} by {} ({})", self.title, self.author, self.year);
        println!("   ISBN: {}, Available: {}", self.isbn, self.available);
    }

    pub fn check_out(&mut self) -> Result<(), &'static str> {
        if self.available {
            self.available = false;
            Ok(())
        } else {
            Err("Book is already check out")
        }
    }

    pub fn return_book(&mut self) {
        self.available = true;
    }

    fn set_genre(&mut self, genre: Genre) {
        self.genre = genre;
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' by {}", self.title, self.author)
    }
}
use crate::book::Book;
use crate::member::{Member, MemberType, MemberId};
use crate::loan::Loan;
use std::collections::HashMap;

pub struct Library {
    name: String,
    books: HashMap<String, Book>,
    members: HashMap<u32, Member>,
    loans: Vec<Loan>,
    next_member_id: u32,
}

impl Library {
    pub fn new(name: String) -> Self {
        Self {
            name,
            books: HashMap::new(),
            members: HashMap::new(),
            loans: Vec::new(),
            next_member_id: 1000,
        }
    }

    pub fn add_book(&mut self, book: Book) {
        let isbn = book.isbn.clone();
        self.books.insert(isbn, book);
    }

    pub fn register_member(&mut self, name: String, email: String, member_type: MemberType) -> u32 {
        let id = self.next_member_id;
        let mut member = Member::new(id, name, email);
        member.member_type = member_type;

        self.members.insert(id, member);
        self.next_member_id += 1;

        id
    }

    pub fn checkout_book(&mut self, isbn: &str, member_id: u32) -> Result<(), String> {
        let book = self.books.get_mut(isbn)
            .ok_or_else(|| "Book not found". to_string())?;

        if !book.available {
            return Err("Book is not available".to_string());
        }

        let member = self.members.get_mut(&member_id)
            .ok_or_else(|| "Member not found".to_string())?;

        if !member.can_borrow() {
            return Err("Member has reached book limit".to_string());
        }

        book.check_out()?;
        member.borrow_book(isbn.to_string())?;

        let loan = Loan::new(isbn.to_string(), MemberId(member_id), 14);
        self.loans.push(loan);

        Ok(())
    }

    pub fn return_book(&mut self, isbn: &str, member_id: u32) -> Result<(), String> {
        let book = self.books.get_mut(isbn)
            .ok_or_else(|| "Book not found".to_string())?;
        book.return_book();

        let member = self.members.get_mut(&member_id)
            .ok_or_else(|| "Member not found".to_string())?;

        if !member.return_book(isbn) {
            return Err("Member didn't have this book".to_string());
        }

        for loan in &mut self.loans {
            if loan.book_isbn == isbn && loan.member_id.0 == member_id && loan.returned.is_none() {
                loan.return_loan("2024-01-20".to_string());
            }
        }

        Ok(())
    }

    pub fn search_books(&self, query: &str) -> Vec<&Book> {
        self.books.values()
            .filter(|book|  {
                book.title.to_lowercase().contains(&query.to_lowercase()) ||
                book.author.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_member_info(&self, id: u32) -> Option<String> {
        if let Some(member) = self.members.get(&id) {
            Some(format!("{} ({} books borrowed)", member.name, member.get_borrowed_count()))
        } else {
            None
        }
    }

    pub fn require_member(&self, id: u32) -> Result<&Member, String> {
        let Some(member) = self.members.get(&id) else {
            return Err(format!("Member {} not found", id));
        };

        Ok(member)
    }

    pub fn display_stats(&self) {
        println!("\n📚 {} Statistics:", self.name);
        println!("Total books: {}", self.books.len());
        println!("Total members: {}", self.members.len());
        println!("Active loans: {}", self.loans.iter().filter(|l| l.returned.is_none()).count());

        let available = self.books.values().filter(|b| b.available).count();
        println!("Available books: {}/{}", available, self.books.len());
    }
}
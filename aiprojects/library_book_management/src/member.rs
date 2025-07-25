use crate::book::Book;

#[derive(Debug)]
pub struct MemberId(pub u32);

pub struct ActiveStatus;

pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub email: String,
    pub member_type: MemberType,
    borrowed_books: Vec<String>,
    pub status: Option<ActiveStatus>,
}

#[derive(Debug)]
pub enum MemberType {
    Student { university: String },
    Senior { age: u32 },
    Regular,
    Staff { employee_id: String },
}

impl Member {
    pub fn new(id: u32, name: String, email: String) -> Self {
        Self {
            id: MemberId(id),
            name,
            email,
            member_type: MemberType::Regular,
            borrowed_books: Vec::new(),
            status: Some(ActiveStatus),
        }
    }

    pub fn get_book_limit(&self) -> u32 {
        match &self.member_type {
            MemberType::Student { .. } => 10,
            MemberType::Senior { age } if *age >= 70 => 8,
            MemberType::Senior { .. } => 6,
            MemberType::Regular => 5,
            MemberType::Staff { .. } => 15,
        }
    }

    pub fn can_borrow(&self) -> bool {
        self.borrowed_books.len() < self.get_book_limit() as usize
    }

    pub fn borrow_book(&mut self, isbn: String) -> Result<(), String> {
        if !self.can_borrow() {
            return Err("Book limit reached".to_string());
        }

        self.borrowed_books.push(isbn);
        Ok(())
    }

    pub fn return_book(&mut self, isbn: &str) -> bool {
        if let Some(pos) = self.borrowed_books.iter().position(|x| x == isbn) {
            self.borrowed_books.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_borrowed_count(&self) -> usize {
        self.borrowed_books.len()
    }
}
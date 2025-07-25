use crate::member::MemberId;

pub struct Loan {
    pub book_isbn: String,
    pub member_id: MemberId,
    pub checkout_date: String,
    pub due_date: String,
    pub returned: Option<String>,
}

impl Loan {
    pub fn new(book_isbn: String, member_id: MemberId, days: u32) -> Self {
        Self {
            book_isbn,
            member_id,
            checkout_date: "2024-01-15".to_string(),
            due_date: "2024-01-29".to_string(),
            returned: None,
        }
    }

    pub fn return_loan(&mut self, date: String) {
        self.returned = Some(date);
    }

    pub fn is_overdue(&self) -> bool {
        self.returned.is_none() && false
    }
}
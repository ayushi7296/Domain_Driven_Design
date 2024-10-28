use crate::domain::models::{Book, LibraryCard, Loan};
use chrono::Utc;

#[derive(Clone)]
pub struct LibraryService;

impl LibraryService {
    pub fn borrow_book(&self, book: &mut Book, member: &LibraryCard) -> Result<Loan, String> {
        if book.available_copies > 0 {
            book.available_copies -= 1;
            Ok(Loan {
                book: book.id.clone(),
                member: member.clone(),
                due_date: Utc::now().date_naive() + chrono::Duration::days(14),
            })
        } else {
            Err("No copies available".to_string())
        }
    }

    pub fn return_book(&self, book: &mut Book, _loan: &Loan) {
        book.available_copies += 1;
    }
}

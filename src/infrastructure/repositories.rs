use crate::domain::models::{Book, ISBN, LibraryCard, Loan, Member};

pub trait BookRepository {
    fn find_by_isbn(&self, isbn: &ISBN) -> Option<Book>;
    fn save(&self, book: &Book);
}

pub trait MemberRepository {
    fn find_by_card(&self, card: &LibraryCard) -> Option<Member>;
    fn save(&self, member: &Member);
}

pub trait LoanRepository {
    fn find_loans_for_member(&self, member: &LibraryCard) -> Vec<Loan>;
    fn save(&self, loan: &Loan);
}

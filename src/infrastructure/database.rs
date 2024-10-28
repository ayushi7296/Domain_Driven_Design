use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::domain::models::{Book, ISBN, LibraryCard, Loan};
use crate::infrastructure::repositories::{BookRepository, MemberRepository, LoanRepository};
use crate::domain::models::Member;

/// In-memory implementation of the BookRepository trait
pub struct InMemoryBookRepository {
    books: Arc<Mutex<HashMap<ISBN, Book>>>,
}

impl InMemoryBookRepository {
    pub fn new() -> Self {
        InMemoryBookRepository {
            books: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl BookRepository for InMemoryBookRepository {
    fn find_by_isbn(&self, isbn: &ISBN) -> Option<Book> {
        self.books.lock().unwrap().get(isbn).cloned()
    }

    fn save(&self, book: &Book) {
        self.books.lock().unwrap().insert(book.id.clone(), book.clone());
    }
}

/// In-memory implementation of the MemberRepository trait
pub struct InMemoryMemberRepository {
    members: Arc<Mutex<HashMap<LibraryCard, Member>>>,
}

impl InMemoryMemberRepository {
    pub fn new() -> Self {
        InMemoryMemberRepository {
            members: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl MemberRepository for InMemoryMemberRepository {
    fn find_by_card(&self, card: &LibraryCard) -> Option<Member> {
        self.members.lock().unwrap().get(card).cloned()
    }

    fn save(&self, member: &Member) {
        self.members.lock().unwrap().insert(member.id.clone(), member.clone());
    }
}

/// In-memory implementation of the LoanRepository trait
pub struct InMemoryLoanRepository {
    loans: Arc<Mutex<HashMap<LibraryCard, Vec<Loan>>>>,
}

impl InMemoryLoanRepository {
    pub fn new() -> Self {
        InMemoryLoanRepository {
            loans: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl LoanRepository for InMemoryLoanRepository {
    fn find_loans_for_member(&self, member: &LibraryCard) -> Vec<Loan> {
        self.loans.lock().unwrap().get(member).cloned().unwrap_or_default()
    }

    fn save(&self, loan: &Loan) {
        let mut loans = self.loans.lock().unwrap();
        loans.entry(loan.member.clone()).or_insert_with(Vec::new).push(loan.clone());
    }
}

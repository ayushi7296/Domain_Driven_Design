use crate::domain::models::{ISBN, LibraryCard};
use crate::domain::services::LibraryService;
use crate::infrastructure::repositories::{BookRepository, MemberRepository, LoanRepository};

pub struct BorrowBookCommand {
    pub isbn: ISBN,
    pub member_id: LibraryCard,
}

pub struct BorrowBookHandler<'a> {
    pub book_repo: &'a dyn BookRepository,
    pub member_repo: &'a dyn MemberRepository,
    pub loan_repo: &'a dyn LoanRepository,
    pub library_service: &'a LibraryService,
}

impl<'a> BorrowBookHandler<'a> {
    pub fn handle(&self, cmd: BorrowBookCommand) -> Result<(), String> {
        let mut book = self
            .book_repo
            .find_by_isbn(&cmd.isbn)
            .ok_or("Book not found")?;
        let member = self
            .member_repo
            .find_by_card(&cmd.member_id)
            .ok_or("Member not found")?;

        let loan = self.library_service.borrow_book(&mut book, &member.id)?;
        self.book_repo.save(&book);
        self.loan_repo.save(&loan);

        Ok(())
    }
}

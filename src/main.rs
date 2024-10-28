mod domain; // Ensure the domain module is accessible
mod infrastructure; // Include the infrastructure module
mod application; // Include the application module if needed

use crate::domain::{services::LibraryService, models::{ISBN, LibraryCard}};
use crate::infrastructure::database::{InMemoryBookRepository,InMemoryLoanRepository,InMemoryMemberRepository}; // Import your HTTP module
use crate::application::commands::{BorrowBookCommand,BorrowBookHandler};
fn main() {
    // Initialize repositories, services, etc.
    let book_repo = InMemoryBookRepository::new();
    let member_repo = InMemoryMemberRepository::new();
    let loan_repo = InMemoryLoanRepository::new();
    let library_service = LibraryService;

    // Create a handler for the borrow book command
    let borrow_book_handler = BorrowBookHandler {
        book_repo: &book_repo,
        member_repo: &member_repo,
        loan_repo: &loan_repo,
        library_service: &library_service,
    };

    // Sample command execution
    let command = BorrowBookCommand {
        isbn: ISBN::new("1234567890123").unwrap(),
        member_id: LibraryCard::new("CARD123"),
    };

    match borrow_book_handler.handle(command) {
        Ok(_) => println!("Book borrowed successfully"),
        Err(e) => println!("Error borrowing book: {}", e),
    }
}

// Represents a Book entity in the Library Management System
#[derive(Debug, Clone)]
pub struct Book {
    pub id: ISBN,
    pub title: String,
    pub author: String,
    pub available_copies: u32,
}

// Represents an ISBN value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ISBN(String);

impl ISBN {
    pub fn new(isbn: &str) -> Result<Self, String> {
        if isbn.len() == 13 {
            Ok(Self(isbn.to_string()))
        } else {
            Err("Invalid ISBN format".to_string())
        }
    }
}

// Represents a Member entity
#[derive(Debug, Clone)]
pub struct Member {
    pub id: LibraryCard,
    pub name: String,
}

// Represents a LibraryCard value object
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryCard(String);

impl LibraryCard {
    pub fn new(card_number: &str) -> Self {
        Self(card_number.to_string())
    }
}

// Represents a Loan entity for a book borrowed by a member
#[derive(Debug, Clone)]
pub struct Loan {
    pub book: ISBN,
    pub member: LibraryCard,
    pub due_date: chrono::NaiveDate,
}

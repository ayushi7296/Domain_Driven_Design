use std::fmt;

/// Custom error type for operations in the library management system
#[derive(Debug)]
pub enum LibraryError {
    /// Error for when a book cannot be borrowed
    BookUnavailable,
    /// Error for when a member is not found in the system
    MemberNotFound,
    /// Error for when a book is not found in the system
    BookNotFound,
    /// Error for general validation issues
    ValidationError(String),
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryError::BookUnavailable => write!(f, "The requested book is not available"),
            LibraryError::MemberNotFound => write!(f, "The specified member could not be found"),
            LibraryError::BookNotFound => write!(f, "The specified book could not be found"),
            LibraryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for LibraryError {}

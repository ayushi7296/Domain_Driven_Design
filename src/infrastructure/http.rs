use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc};
use crate::domain::models::{ISBN, LibraryCard};
use crate::domain::services::LibraryService;
use crate::infrastructure::repositories::{BookRepository, MemberRepository, LoanRepository};
use crate::domain::models::Loan;

#[derive(Clone)]
pub struct AppState {
    pub library_service: LibraryService,
    pub book_repo: Arc<dyn BookRepository + Send + Sync>,
    pub member_repo: Arc<dyn MemberRepository + Send + Sync>,
    pub loan_repo: Arc<dyn LoanRepository + Send + Sync>,
}

#[derive(Deserialize)]
struct BorrowRequest {
    member_id: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    data: Option<T>,
    message: String,
}

/// Handler for borrowing a book
async fn borrow_book(
    State(state): State<AppState>,
    Path(isbn): Path<String>,
    Json(payload): Json<BorrowRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut book = state
        .book_repo
        .find_by_isbn(&ISBN::new(&isbn).map_err(|_| StatusCode::BAD_REQUEST)?)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let member = state
        .member_repo
        .find_by_card(&LibraryCard::new(&payload.member_id))
        .ok_or(StatusCode::NOT_FOUND)?;
    
    match state.library_service.borrow_book(&mut book, &member.id) {
        Ok(loan) => {
            state.book_repo.save(&book);
            state.loan_repo.save(&loan);
            Ok(Json(ApiResponse {
                data: Some(format!("Loan created for book ISBN: {}", isbn)),
                message: "Book borrowed successfully".to_string(),
            }))
        }
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

/// Handler for returning a book
async fn return_book(
    State(state): State<AppState>,
    Path(isbn): Path<String>,
    Json(payload): Json<BorrowRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut book = state
        .book_repo
        .find_by_isbn(&ISBN::new(&isbn).map_err(|_| StatusCode::BAD_REQUEST)?)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let book_id = book.id.clone();
    state.library_service.return_book(&mut book, &Loan {
        book: book_id,
        member: LibraryCard::new(&payload.member_id),
        due_date: chrono::Utc::now().date_naive(),
    });
    state.book_repo.save(&book);
    
    Ok(Json(ApiResponse {
        data: None,
        message: "Book returned successfully".to_string(),
    }))
}

/// Initialize and set up routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/borrow/:isbn", post(borrow_book))
        .route("/return/:isbn", post(return_book))
        .with_state(state)
}

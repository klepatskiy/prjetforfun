use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Database error:")]
    NotFound,
    #[error("Parse url error:")]
    URLParseError,
}

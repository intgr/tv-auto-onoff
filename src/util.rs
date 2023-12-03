use std::error::Error;

/// Alias for a type-erased error type.
/// Stolen from https://docs.rs/axum/latest/axum/type.BoxError.html
pub type BoxError = Box<dyn Error + Sync + Send>;

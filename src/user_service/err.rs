use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, UserService>; 

#[derive(Debug, Error)]
enum UserService {
    #[error("Internal database error")]
    DataBaseError(#[from] sqlx::Error)
}

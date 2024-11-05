use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("Database error: {0}")]
    DatabaseError(DatabaseError),
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("File error: {0}")]
    FileError(String),
    #[error("Address parsing error: {0}")]
    AddressParsingError(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    Generic(String),
    #[error("Existing database item: {0}")]
    ExistingItem(String),
}

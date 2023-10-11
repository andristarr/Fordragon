#[derive(Debug)]
pub enum Error {
    InvalidArguments(String),
    DatabaseError(DatabaseError),
    ParsingError(String),
    FileError(String),
    AddressParsingError(String),
    RuntimeError(String)
}

#[derive(Debug)]
pub enum DatabaseError {
    Generic(String),
    ExistingItem(String)
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::DatabaseError(DatabaseError::Generic(value.to_string()))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::ParsingError(value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::FileError(value.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(value: std::net::AddrParseError) -> Self {
        Error::AddressParsingError(value.to_string())
    }
}
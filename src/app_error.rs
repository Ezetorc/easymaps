use nbt_rs::error::ParseError;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    NBTParseError(ParseError),
    NBTMissingField(String),
    ParseError(String),
    IOFileError(std::io::Error),
    NotFound(String),
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::NBTParseError(error) => Some(error),
            AppError::NBTMissingField(_) => None,
            AppError::ParseError(_) => None,
            AppError::IOFileError(error) => Some(error),
            AppError::NotFound(_) => None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NBTParseError(error) => write!(f, "Error parsing NBT: {}", error),
            AppError::NBTMissingField(message) => {
                write!(f, "Missing field in NBT parsing: {message}")
            }
            AppError::ParseError(message) => write!(f, "Error while parsing: {message}"),
            AppError::IOFileError(error) => write!(f, "Error while I/O process: {error}"),
            AppError::NotFound(message) => write!(f, "{message}"),
        }
    }
}

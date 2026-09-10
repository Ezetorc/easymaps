use nbt_rs::error::ParseError;
use std::{fmt::Display, io::Error};

pub enum AppError {
    NBTParseError(ParseError),
    NBTMissingField(String),
    ParseError(String),
    IOFileError(Error),
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
        }
    }
}

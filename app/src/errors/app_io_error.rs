use std::{fmt::Display, io};

use crate::errors::app_error::AppError;

#[derive(Debug)]
pub enum AppIoError {
    Generic(io::Error),
    ExtractionFailed(String),
    NameError(String),
    InvalidEntries(String),
    InvalidPath(String),
}

impl Display for AppIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(error) => write!(f, "{error}"),
            Self::ExtractionFailed(message) => write!(f, "Error while extracting file: {message}"),
            Self::NameError(message) => write!(f, "Error with file name: {message}"),
            Self::InvalidEntries(message) => write!(f, "Invalid entries: {message}"),
            Self::InvalidPath(message) => write!(f, "Invalid path: {message}"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::Io(AppIoError::Generic(value))
    }
}

use mc_launcher_core::LauncherError;
use nbt_rs::error::ParseError;
use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum AppError {
    NBTParseError(ParseError),
    NBTMissingField(String),
    ParseError(String),
    IOError(std::io::Error),
    NotFound(String),
    MinecraftLauncherError(LauncherError),
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::NBTParseError(error) => Some(error),
            AppError::NBTMissingField(_) => None,
            AppError::ParseError(_) => None,
            AppError::IOError(error) => Some(error),
            AppError::NotFound(_) => None,
            AppError::MinecraftLauncherError(error) => Some(error),
        }
    }
}

impl From<LauncherError> for AppError {
    fn from(value: LauncherError) -> Self {
        Self::MinecraftLauncherError(value)
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
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
            AppError::IOError(error) => write!(f, "Error while I/O process: {error}"),
            AppError::NotFound(message) => write!(f, "{message}"),
            AppError::MinecraftLauncherError(error) => {
                write!(f, "Error while launching Minecraft: {error}")
            }
        }
    }
}

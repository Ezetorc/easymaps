use std::{fmt::Display, io};

use crate::errors::{
    app_messaging_error::AppMessagingError, app_minecraft_error::AppMinecraftError,
};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Minecraft(AppMinecraftError),
    Messaging(AppMessagingError),
    Generic(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(error) => write!(f, "[I/O error] {error}"),
            AppError::Messaging(error) => write!(f, "[Messaging error] {error}"),
            AppError::Minecraft(error) => write!(f, "[Minecraft error] {error}"),
            AppError::Generic(message) => write!(f, "[Error] {message}"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

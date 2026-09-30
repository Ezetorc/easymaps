use std::fmt::Display;

use crate::errors::{
    app_io_error::AppIoError, app_messaging_error::AppMessagingError,
    app_minecraft_error::AppMinecraftError,
};

#[derive(Debug)]
pub enum AppError {
    Io(AppIoError),
    Minecraft(AppMinecraftError),
    Messaging(AppMessagingError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Io(error) => write!(f, "[I/O error] {error}"),
            AppError::Messaging(error) => write!(f, "[Messaging error] {error}"),
            AppError::Minecraft(error) => write!(f, "[Minecraft error] {error}"),
        }
    }
}

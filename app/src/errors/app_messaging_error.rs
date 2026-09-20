use std::fmt::Display;

#[derive(Debug)]
pub enum AppMessagingError {
    InvalidRequest(String),
    ParseRequestError(String),
}

impl Display for AppMessagingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(message) => write!(f, "Invalid request: {message}"),
            Self::ParseRequestError(message) => write!(f, "Error parsing request: {message}"),
        }
    }
}

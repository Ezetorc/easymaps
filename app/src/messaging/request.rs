use serde::Deserialize;
use std::io::{self, ErrorKind, Read};

use crate::errors::{app_error::AppError, app_messaging_error::AppMessagingError};

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Start {
        minecraft_version: String,
        filename: String,
    },
}

impl Request {
    pub const MAX_REQUEST_SIZE: usize = 256;

    pub fn read() -> Result<Option<Self>, AppError> {
        let Some(length) = Self::read_length()? else {
            return Ok(None);
        };

        let buffer = Self::read_message(length)?;
        let request = Self::parse_message(buffer)?;

        Ok(Some(request))
    }

    fn read_length() -> Result<Option<usize>, AppError> {
        let mut length_buffer = [0u8; 4];
        let mut bytes_read = 0;

        while bytes_read < length_buffer.len() {
            match io::stdin().read(&mut length_buffer[bytes_read..]) {
                Ok(0) => {
                    if bytes_read == 0 {
                        return Ok(None);
                    }

                    return Err(AppError::Io(io::Error::new(
                        ErrorKind::UnexpectedEof,
                        "Incomplete request length",
                    )));
                }

                Ok(n) => {
                    bytes_read += n;
                }

                Err(error) => {
                    return Err(AppError::Io(error));
                }
            }
        }

        Ok(Some(u32::from_le_bytes(length_buffer) as usize))
    }

    fn read_message(length: usize) -> Result<Vec<u8>, AppError> {
        if length > Self::MAX_REQUEST_SIZE {
            return Err(AppError::Messaging(AppMessagingError::InvalidRequest(
                "Request is too big".to_string(),
            )));
        }

        let mut request_buffer = vec![0u8; length];

        io::stdin()
            .read_exact(&mut request_buffer)
            .map_err(AppError::Io)?;

        Ok(request_buffer)
    }

    fn parse_message(request_buffer: Vec<u8>) -> Result<Self, AppError> {
        serde_json::from_slice::<Self>(&request_buffer).map_err(|error| {
            AppError::Messaging(AppMessagingError::ParseRequestError(error.to_string()))
        })
    }
}

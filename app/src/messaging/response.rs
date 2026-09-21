use serde::Serialize;
use std::io::{self, Write};

use crate::errors::app_error::AppError;

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    Starting,
    Finished,
    Error { error: String },
}

impl Response {
    pub fn send(&self) -> Result<(), AppError> {
        let response_json = serde_json::to_vec(self).map_err(io::Error::other)?;
        let response_length = (response_json.len() as u32).to_le_bytes();
        let mut stdout = io::stdout().lock();

        stdout.write_all(&response_length)?;
        stdout.write_all(&response_json)?;
        stdout.flush()?;

        Ok(())
    }
}

use serde::Serialize;
use std::io::{self, Write, stdout};

use crate::errors::app_error::AppError;

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    Starting { web_tab_id: Option<i32> },
    Finished { web_tab_id: Option<i32> },
    AppError { web_tab_id: Option<i32> },
    ConnectionError,
}

impl Response {
    pub fn send(&self) -> Result<(), AppError> {
        let response_json = serde_json::to_vec(self).map_err(io::Error::other)?;
        let response_length = response_json.len() as u32;
        let response_length = response_length.to_le_bytes();
        let mut stdout = stdout().lock();

        stdout.write_all(&response_length)?;
        stdout.write_all(&response_json)?;
        stdout.flush()?;

        Ok(())
    }
}

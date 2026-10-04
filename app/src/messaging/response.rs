use anyhow::Result;
use serde::Serialize;
use std::io::{self, Write, stdout};

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    Importing { requester_id: i32 },
    Installing { requester_id: i32 },
    Launching { requester_id: i32 },
    AppError { requester_id: i32 },
    ConnectionError,
}

impl Response {
    pub fn send(&self) -> Result<()> {
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

use anyhow::{Context, Result};
use serde::Serialize;
use std::io::{Write, stdout};

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    Importing { requester_id: i32 },
    Installing { requester_id: i32 },
    Launching { requester_id: i32 },
    AppError { requester_id: i32, error: AppError },
    ConnectionError,
}

#[derive(Debug, Serialize)]
pub enum AppError {
    Unexpected,
    BedrockWorldNotSupported,
    UnknownWorldVersion,
}

impl Response {
    pub fn send(&self) -> Result<()> {
        let response_json =
            serde_json::to_vec(&self).with_context(|| format!("Converting {self:?}"))?;
        let response_length = response_json.len() as u32;
        let response_length = response_length.to_le_bytes();
        let mut stdout = stdout().lock();

        stdout
            .write_all(&response_length)
            .with_context(|| format!("Writing to StdOut with {self:?}"))?;
        stdout
            .write_all(&response_json)
            .with_context(|| format!("Writing to StdOut with {self:?}"))?;
        stdout
            .flush()
            .with_context(|| format!("Flushing StdOut with {self:?}"))?;

        Ok(())
    }
}

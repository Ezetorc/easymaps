use anyhow::{Context, Result};
use serde::Serialize;
use std::io::{Write, stdout};

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    ImportingMap {
        requester_id: i32,
    },
    InstallingVersion {
        requester_id: i32,
    },
    LaunchingMinecraft {
        requester_id: i32,
    },
    CheckingMap {
        requester_id: i32,
    },
    MapCheckResult {
        requester_id: i32,
        world_exists: bool,
    },
    AppError {
        requester_id: i32,
        error: AppError,
    },
    DeletingMap {
        requester_id: i32,
    },
    MapDeleteSuccess {
        requester_id: i32,
    },
    ConnectionError,
}

#[derive(Debug, Serialize)]
pub enum AppError {
    Unexpected,
    BedrockWorldNotSupported,
    UnknownWorldVersion,
    WorldNotFound,
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

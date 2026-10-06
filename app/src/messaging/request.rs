use serde::Deserialize;
use std::io::{Read, stdin};

use anyhow::{Context, Result, bail};

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Start {
        minecraft_version: Option<String>,
        download_path: String,
        requester_id: i32,
    },
}

impl Request {
    pub const MAX_REQUEST_SIZE: usize = 256;

    pub fn read() -> Result<Option<Self>> {
        let Some(length) = Self::read_length()? else {
            return Ok(None);
        };

        let buffer = Self::read_message(length)?;
        let request = Self::parse_message(buffer)?;

        Ok(Some(request))
    }

    fn read_length() -> Result<Option<usize>> {
        let mut length_buffer = [0u8; 4];
        let mut bytes_read = 0;

        while bytes_read < length_buffer.len() {
            let read_result = stdin()
                .read(&mut length_buffer[bytes_read..])
                .context("Reading length of buffer")?;

            if read_result == 0 {
                match bytes_read {
                    0 => return Ok(None),
                    _ => {
                        bail!("Incomplete request length")
                    }
                }
            } else {
                bytes_read += read_result;
            }
        }

        Ok(Some(u32::from_le_bytes(length_buffer) as usize))
    }

    fn read_message(length: usize) -> Result<Vec<u8>> {
        if length > Self::MAX_REQUEST_SIZE {
            bail!("Request is too big")
        }

        let mut request_buffer = vec![0u8; length];

        stdin().read_exact(&mut request_buffer)?;

        Ok(request_buffer)
    }

    fn parse_message(request_buffer: Vec<u8>) -> Result<Self> {
        serde_json::from_slice::<Self>(&request_buffer)
            .context("Parsing request buffer to JSON format")
    }
}

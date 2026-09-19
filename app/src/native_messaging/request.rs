use serde::Deserialize;
use std::io::{self, Read};

use crate::{native_messaging::response::Response, utilities::log};

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Start {
        minecraft_version: String,
        filename: String,
    },
}

impl Request {
    pub fn get() -> Option<Self> {
        let length = Self::length()?;

        let mut request_buffer = vec![0u8; length];
        if let Err(error) = io::stdin().read_exact(&mut request_buffer) {
            log(&format!("[Message read error] {error}"));
            return None;
        }

        match serde_json::from_slice(&request_buffer) {
            Ok(request) => request,
            Err(error) => {
                log(&format!("[JSON error] {error}"));
                let _ = &Response::Error {
                    error: error.to_string(),
                }
                .send();
                None
            }
        }
    }

    pub fn length() -> Option<usize> {
        let mut length_buffer = [0u8; 4];

        if let Err(error) = io::stdin().read_exact(&mut length_buffer) {
            log(&format!("[Stdin error]: {error}"));
            return None;
        }

        Some(u32::from_le_bytes(length_buffer) as usize)
    }
}

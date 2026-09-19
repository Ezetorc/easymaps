use serde::Serialize;
use std::io::{self, Write};

#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum Response {
    Starting,
    Finished,
    Error { error: String },
}

impl Response {
    pub fn send(&self) -> io::Result<()> {
        let response_json = serde_json::to_vec(self).map_err(io::Error::other)?;
        let response_length = (response_json.len() as u32).to_le_bytes();
        let stdout = io::stdout();
        let mut stdout = stdout.lock();

        stdout.write_all(&response_length)?;
        stdout.write_all(&response_json)?;
        stdout.flush()?;

        Ok(())
    }
}

use std::{fs, path::Path};

use serde::Deserialize;

use crate::app_error::AppError;

#[derive(Deserialize)]
pub struct EasyMapsConfig {
    pub minecraft_version: String,
}

impl EasyMapsConfig {
    pub fn from_file(path: &Path) -> Result<Self, AppError> {
        let file_content = fs::read_to_string(path).map_err(AppError::IOFileError)?;
        let config: Self = serde_json::from_str(&file_content)
            .map_err(|_| AppError::ParseError("Error parsing 'easymaps.json' file".to_string()))?;

        Ok(config)
    }
}

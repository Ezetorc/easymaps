use std::{env, path::PathBuf};

use crate::errors::app_error::AppError;

pub struct AppPaths;

impl AppPaths {
    pub fn root() -> Result<PathBuf, AppError> {
        let Some(local_app_data) = env::var_os("LOCALAPPDATA") else {
            return Err(AppError::Generic(String::from(
                "Local App Data directory not found",
            )));
        };

        Ok(PathBuf::from(local_app_data).join("EasyMaps"))
    }

    pub fn minecraft() -> Result<PathBuf, AppError> {
        Ok(Self::root()?.join("Minecraft"))
    }

    pub fn worlds() -> Result<PathBuf, AppError> {
        Ok(Self::minecraft()?.join("saves"))
    }

    pub fn world(name: &str) -> Result<PathBuf, AppError> {
        Ok(Self::worlds()?.join(name))
    }
}

use std::{
    env,
    path::{Path, PathBuf},
};

use crate::errors::{app_error::AppError, app_io_error::AppIoError};

pub struct AppPaths;

impl AppPaths {
    pub fn root() -> Result<PathBuf, AppError> {
        let Some(local_app_data) = env::var_os("LOCALAPPDATA") else {
            return Err(AppError::Io(AppIoError::InvalidPath(
                "Local App Data directory not found".to_string(),
            )));
        };

        Ok(PathBuf::from(local_app_data).join("EasyMaps"))
    }

    pub fn temp() -> Result<PathBuf, AppError> {
        Ok(Self::root()?.join("Temp"))
    }

    pub fn minecraft() -> Result<PathBuf, AppError> {
        Ok(Self::root()?.join("Minecraft"))
    }

    pub fn worlds() -> Result<PathBuf, AppError> {
        Ok(Self::minecraft()?.join("saves"))
    }

    pub fn world<P: AsRef<Path>>(name: P) -> Result<PathBuf, AppError> {
        Ok(Self::worlds()?.join(name))
    }
}

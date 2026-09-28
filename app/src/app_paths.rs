use std::{
    env,
    path::{Path, PathBuf},
};

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

    pub fn world_level_dat<P: AsRef<Path>>(name: P) -> Result<PathBuf, AppError> {
        Ok(Self::world(name)?.join("level.dat"))
    }
}

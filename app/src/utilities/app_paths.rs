use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
pub struct AppPaths;

impl AppPaths {
    pub fn root() -> Result<PathBuf> {
        let local_app_data =
            env::var_os("LOCALAPPDATA").context("Local App Data directory not found")?;

        Ok(PathBuf::from(local_app_data).join("EasyMaps"))
    }

    pub fn temp() -> Result<PathBuf> {
        Ok(Self::root()?.join("Temp"))
    }

    pub fn minecraft() -> Result<PathBuf> {
        Ok(Self::root()?.join("Minecraft"))
    }

    pub fn worlds() -> Result<PathBuf> {
        Ok(Self::minecraft()?.join("saves"))
    }

    pub fn world<P: AsRef<Path>>(name: P) -> Result<PathBuf> {
        Ok(Self::worlds()?.join(name))
    }
}

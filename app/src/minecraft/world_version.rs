use flate2::read::GzDecoder;
use nbt_rs::{
    get_field, parse_nbt,
    types::{NbtCompound, NbtString, NbtTag},
};
use std::{io::Read, path::Path};

use crate::{
    app_paths::AppPaths,
    errors::{app_error::AppError, app_minecraft_error::AppMinecraftError},
};

#[derive(Debug)]
pub struct WorldVersion;

impl WorldVersion {
    pub fn from_world(world_name: &str) -> Result<String, AppError> {
        let level_dat_path = AppPaths::world_level_dat(world_name)?;
        let world_version = Self::from_level_dat(&level_dat_path)?;

        Ok(world_version)
    }

    fn from_level_dat(path: &Path) -> Result<String, AppError> {
        let nbt = Self::read_compressed_nbt(path)?;
        let (_, compound) = Self::parse_nbt(&nbt)?;
        let name = Self::extract_version_name(&compound)?;

        match name {
            NbtTag::String(name) => Ok(name.to_string()),
            _ => Err(AppError::Minecraft(AppMinecraftError::Nbt(
                "Couldn't parse version data".to_string(),
            ))),
        }
    }

    fn extract_version_name(compound: &NbtCompound) -> Result<&NbtTag, AppError> {
        get_field!(compound, "Data"."Version"."Name").ok_or(AppError::Minecraft(
            AppMinecraftError::Nbt("Missing version name".to_string()),
        ))
    }

    fn parse_nbt(nbt: &[u8]) -> Result<(NbtString, NbtCompound), AppError> {
        parse_nbt(nbt).map_err(|error| {
            AppError::Minecraft(AppMinecraftError::Nbt(format!(
                "Error parsing NBT: {error}"
            )))
        })
    }

    fn read_compressed_nbt(path: &Path) -> Result<Vec<u8>, AppError> {
        let file = std::fs::File::open(path).map_err(AppError::Io)?;
        let mut gzip_decoder = GzDecoder::new(file);
        let mut nbt = Vec::new();

        gzip_decoder.read_to_end(&mut nbt)?;

        Ok(nbt)
    }
}

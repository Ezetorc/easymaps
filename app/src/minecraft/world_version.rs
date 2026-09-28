use flate2::read::GzDecoder;
use nbt_rs::{
    get_field, parse_nbt,
    types::{NbtCompound, NbtString, NbtTag},
};
use std::{io::Read, path::Path};

use crate::{
    app_paths::AppPaths,
    errors::{
        app_error::AppError, app_io_error::AppIoError, app_minecraft_error::AppMinecraftError,
    },
};

#[derive(Debug)]
pub struct WorldVersion;

impl WorldVersion {
    pub fn from_world<P: AsRef<Path>>(world_name: P) -> Result<Option<String>, AppError> {
        let level_dat_path = AppPaths::world_level_dat(world_name)?;
        let nbt = Self::read_compressed_nbt(level_dat_path)?;
        let (_, compound) = Self::parse_nbt(&nbt)?;
        let name = Self::extract_version_name(&compound);

        if let Some(name) = name {
            match name {
                NbtTag::String(name) => return Ok(Some(name.to_string())),
                _ => return Ok(None),
            };
        }

        Ok(None)
    }

    fn extract_version_name(compound: &NbtCompound) -> Option<&NbtTag> {
        get_field!(compound, "Data"."Version"."Name")
    }

    fn parse_nbt(nbt: &[u8]) -> Result<(NbtString, NbtCompound), AppError> {
        parse_nbt(nbt).map_err(|error| {
            AppError::Minecraft(AppMinecraftError::Nbt(format!(
                "Error parsing NBT: {error}"
            )))
        })
    }

    fn read_compressed_nbt<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, AppError> {
        let file =
            std::fs::File::open(path).map_err(|error| AppError::Io(AppIoError::Generic(error)))?;

        let mut gzip_decoder = GzDecoder::new(file);
        let mut nbt = Vec::new();

        gzip_decoder.read_to_end(&mut nbt)?;

        Ok(nbt)
    }
}

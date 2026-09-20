use flate2::read::GzDecoder;
use nbt_rs::{get_field, parse_nbt, types::NbtTag};
use std::{io::Read, path::Path};

use crate::errors::{app_error::AppError, app_minecraft_error::AppMinecraftError};

#[derive(Debug)]
pub struct WorldVersion;

impl WorldVersion {
    pub fn from_world(path: &Path) -> Result<String, AppError> {
        let nbt = Self::decode_nbt_file(path)?;

        let (_, compound) = parse_nbt(&nbt).map_err(|error| {
            AppError::Minecraft(AppMinecraftError::Nbt(format!("NBT parse error {error}")))
        })?;

        let name = get_field!(compound, "Data"."Version"."Name").ok_or(AppError::Minecraft(
            AppMinecraftError::Nbt("Missing version name".to_string()),
        ))?;

        match name {
            NbtTag::String(name) => Ok(name.to_string()),
            _ => Err(AppError::Minecraft(AppMinecraftError::Nbt(
                "Couldn't parse version data".to_string(),
            ))),
        }
    }

    fn decode_nbt_file(path: &Path) -> Result<Vec<u8>, AppError> {
        let file = std::fs::File::open(path).map_err(AppError::Io)?;
        let mut gzip_decoder = GzDecoder::new(file);
        let mut nbt = Vec::new();

        gzip_decoder.read_to_end(&mut nbt)?;

        Ok(nbt)
    }
}

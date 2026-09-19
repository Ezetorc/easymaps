use flate2::read::GzDecoder;
use nbt_rs::{get_field, parse_nbt, types::NbtTag};
use std::{io::Read, path::Path};

use crate::app_error::AppError;

#[derive(Debug)]
pub struct WorldVersion(String);

impl WorldVersion {
    pub fn version(&self) -> &String {
        &self.0
    }

    pub fn from_world(path: &Path, version_fallback: String) -> Result<Self, AppError> {
        let version_from_level_dat = Self::from_level_dat(&path.join("level.dat"));

        if let Ok(version) = version_from_level_dat {
            return Ok(version);
        }

        Ok(WorldVersion(version_fallback))
    }

    fn from_level_dat(path: &Path) -> Result<Self, AppError> {
        let nbt = Self::decode_nbt_file(path)?;
        let (_, compound) = parse_nbt(&nbt).map_err(AppError::NBTParseError)?;

        let name = get_field!(compound, "Data"."Version"."Name").ok_or(
            AppError::NBTMissingField("Missing version name".to_string()),
        )?;

        match name {
            NbtTag::String(name) => Ok(Self(name.to_string())),
            _ => Err(AppError::ParseError(
                "Couldn't parse version data".to_string(),
            )),
        }
    }

    fn decode_nbt_file(path: &Path) -> Result<Vec<u8>, AppError> {
        let file = std::fs::File::open(path).map_err(AppError::IOError)?;
        let mut gzip_decoder = GzDecoder::new(file);
        let mut nbt = Vec::new();

        gzip_decoder.read_to_end(&mut nbt)?;

        Ok(nbt)
    }
}

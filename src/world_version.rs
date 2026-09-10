use flate2::read::GzDecoder;
use nbt_rs::{get_field, parse_nbt, types::NbtTag};
use std::io::Read;

use crate::app_error::AppError;

#[derive(Debug)]
pub struct WorldVersion {
    pub id: i32,
    pub name: String,
}

impl WorldVersion {
    pub fn from_level_dat(path: &str) -> Result<Self, AppError> {
        let nbt = Self::decode_nbt_file(path)?;
        let (_, compound) = parse_nbt(&nbt).map_err(AppError::NBTParseError)?;

        let id = get_field!(compound, "Data"."Version"."Id")
            .ok_or(AppError::NBTMissingField("Missing version ID".to_string()))?;

        let name = get_field!(compound, "Data"."Version"."Name").ok_or(
            AppError::NBTMissingField("Missing version name".to_string()),
        )?;

        match (id, name) {
            (NbtTag::Int(id), NbtTag::String(name)) => Ok(Self {
                id: *id,
                name: name.to_string(),
            }),
            _ => Err(AppError::ParseError(
                "Couldn't parse version data".to_string(),
            )),
        }
    }

    fn decode_nbt_file(path: &str) -> Result<Vec<u8>, AppError> {
        let file = std::fs::File::open(path).map_err(AppError::IOFileError)?;
        let mut gzip_decoder = GzDecoder::new(file);
        let mut nbt = Vec::new();

        gzip_decoder
            .read_to_end(&mut nbt)
            .map_err(AppError::IOFileError)?;

        Ok(nbt)
    }
}

use crate::utilities::read_compressed_nbt::read_compressed_nbt;
use anyhow::Result;
use nbt_rs::{get_field, parse_nbt, types::NbtTag};
use std::path::PathBuf;

pub struct World {
    name: String,
    path: PathBuf,
}

impl World {
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn find_version(&self) -> Result<Option<String>> {
        let level_dat_path = self.path.join("level.dat");
        let nbt = read_compressed_nbt(level_dat_path)?;
        let (_, compound) = parse_nbt(&nbt)?;
        let name = get_field!(compound, "Data"."Version"."Name");

        if let Some(name) = name {
            match name {
                NbtTag::String(name) => return Ok(Some(name.to_string())),
                _ => return Ok(None),
            };
        }

        Ok(None)
    }
}

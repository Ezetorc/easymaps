use crate::{
    minecraft::world::World,
    utilities::{
        app_paths::AppPaths, extract_compressed_file::extract_compressed_file,
        path_extension::PathExtension,
    },
};
use anyhow::{Context, Result, bail};
use std::{
    ffi::OsStr,
    fs::{create_dir, read_dir, rename},
    path::{Path, PathBuf},
};

pub struct WorldImporter;

impl WorldImporter {
    pub fn import(path: &PathBuf) -> Result<World> {
        let temp_directory_path = AppPaths::temp()?;

        temp_directory_path.clean_directory()?;

        Self::move_input_to_temp_directory(path.to_path_buf())?;

        let level_dat_path = temp_directory_path
            .find_file("level.dat")?
            .context("Downloaded folder is not a Minecraft world")?;
        let world_directory_path = level_dat_path
            .parent()
            .context("Couldn't get file path's parent")?;
        let world_name = world_directory_path
            .file_name()
            .and_then(OsStr::to_str)
            .context("Couldn't get directory's file name")?;
        let world_path = AppPaths::world(world_name)?;

        rename(world_directory_path, &world_path)
            .with_context(|| format!("Moving '{world_directory_path:?}' to '{world_path:?}'"))?;

        temp_directory_path
            .clean_directory()
            .with_context(|| format!("Cleaning directory '{path:?}'"))?;

        Ok(World::new(world_name, world_path))
    }

    fn move_input_to_temp_directory(path: PathBuf) -> Result<()> {
        let temp_directory_path = AppPaths::temp()?;

        if path.is_dir() {
            rename(path, &temp_directory_path)?;
        } else if path.is_file() {
            extract_compressed_file(path, &temp_directory_path)?;
            Self::normalize_entries(&temp_directory_path)?;
        }

        Ok(())
    }

    fn normalize_entries(directory_path: &Path) -> Result<()> {
        let directory_entries = read_dir(directory_path)?.collect::<Result<Vec<_>, _>>()?;
        let entries_count = directory_entries.len();

        match entries_count {
            0 => {
                bail!("Expected at least 1 entry inside downloaded file, found 0")
            }
            1 => {
                let entry = &directory_entries[0];

                if entry.path().is_file() {
                    let new_directory_path = &directory_path.join("Extracted");

                    create_dir(new_directory_path)?;
                    new_directory_path.move_entry_inside(entry)?;
                }
            }
            2.. => {
                let new_directory_path = &directory_path.join("Extracted");

                create_dir(new_directory_path)?;

                for entry in directory_entries {
                    new_directory_path.move_entry_inside(&entry)?;
                }
            }
        }

        Ok(())
    }
}

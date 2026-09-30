use std::{
    ffi::OsStr,
    fs::{create_dir, read_dir, rename},
    path::{Path, PathBuf},
};

use crate::{
    app_paths::AppPaths,
    errors::{
        app_error::AppError, app_io_error::AppIoError, app_minecraft_error::AppMinecraftError,
    },
    minecraft::world::World,
    utilities::{
        clean_directory::clean_directory, extract_compressed_file::extract_compressed_file,
        find_file::find_file, move_entry_to::move_entry_to,
    },
};

pub struct WorldImporter;

impl WorldImporter {
    pub fn import(path: PathBuf) -> Result<World, AppError> {
        let temp_directory_path = AppPaths::temp()?;

        Self::move_input_to_temp_directory(path)?;

        let level_dat = find_file(&temp_directory_path, "level.dat")?.ok_or_else(|| {
            AppError::Minecraft(AppMinecraftError::World(
                "Downloaded folder is not a Minecraft world".to_string(),
            ))
        })?;

        let world_directory_path = level_dat.parent().ok_or_else(|| {
            AppError::Io(AppIoError::NameError(
                "Couldn't get file path's parent".to_string(),
            ))
        })?;

        let world_name = world_directory_path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| {
                AppError::Io(AppIoError::NameError(
                    "Couldn't get directory's file name".to_string(),
                ))
            })?;
        let world_path = AppPaths::world(world_name)?;

        rename(world_directory_path, &world_path)?;
        clean_directory(&temp_directory_path)?;

        Ok(World::new(world_name, world_path))
    }

    fn move_input_to_temp_directory(path: PathBuf) -> Result<(), AppError> {
        let temp_directory_path = AppPaths::temp()?;

        if path.is_dir() {
            rename(path, &temp_directory_path)?;
        } else if path.is_file() {
            extract_compressed_file(path, &temp_directory_path)?;
            Self::normalize_entries(&temp_directory_path)?;
        }

        Ok(())
    }

    fn normalize_entries(directory_path: &Path) -> Result<(), AppError> {
        let directory_entries = read_dir(directory_path)?.collect::<Result<Vec<_>, _>>()?;
        let entries_count = directory_entries.len();

        match entries_count {
            0 => {
                return Err(AppError::Io(AppIoError::InvalidEntries(
                    "Expected at least 1 entry inside downloaded file, found 0".to_string(),
                )));
            }
            1 => {
                let entry = &directory_entries[0];

                if entry.path().is_file() {
                    let new_directory_path = &directory_path.join("Extracted");

                    create_dir(new_directory_path)?;
                    move_entry_to(entry, new_directory_path)?;
                }
            }
            2.. => {
                let new_directory_path = &directory_path.join("Extracted");

                create_dir(new_directory_path)?;

                for entry in directory_entries {
                    move_entry_to(&entry, new_directory_path)?;
                }
            }
        }

        Ok(())
    }
}

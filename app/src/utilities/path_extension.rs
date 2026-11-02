use std::{
    ffi::OsStr,
    fs::{DirEntry, read_dir, remove_dir_all, remove_file, rename},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use coxswain_core::archive;

use crate::utilities::extract_compressed_file::extract_compressed_file;

pub trait PathExtension {
    fn clean_directory(&self) -> Result<()>;
    fn find_file(&self, name: &str, extract_if_necessary: bool) -> Result<Option<PathBuf>>;
    fn find_directory(&self, name: &str) -> Result<Option<PathBuf>>;
    fn has_extension(&self, extension: &str) -> bool;
    fn move_entry_inside(&self, entry: &DirEntry) -> Result<()>;
}

impl PathExtension for Path {
    fn clean_directory(&self) -> Result<()> {
        let entries = read_dir(self)?.collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let entry_path = entry.path();

            if entry_path.is_file() {
                remove_file(entry_path)?;
            } else if entry_path.is_dir() {
                remove_dir_all(entry_path)?;
            }
        }

        Ok(())
    }

    fn find_directory(&self, name: &str) -> Result<Option<PathBuf>> {
        let entries = read_dir(self)?.collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                if entry.file_name() == name {
                    return Ok(Some(entry_path));
                }

                if let Some(result) = entry_path.find_directory(name)? {
                    return Ok(Some(result));
                }
            }
        }

        Ok(None)
    }

    fn find_file(&self, name: &str, extract_if_necessary: bool) -> Result<Option<PathBuf>> {
        let entries = read_dir(self)?.collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if entry.file_name() == name {
                    return Ok(Some(entry_path));
                }

                if !extract_if_necessary {
                    continue;
                }

                let Ok(new_entries) = archive::list_in(&entry_path, "") else {
                    continue;
                };

                for new_entry in new_entries {
                    let new_entry_path = Path::new(&new_entry.name);

                    if new_entry_path
                        .file_name()
                        .is_some_and(|entry_name| entry_name == name)
                    {
                        let destination = entry_path
                            .parent()
                            .context("Archive has no parent directory")?;

                        extract_compressed_file(entry_path.clone(), destination)?;

                        let extracted_path = destination.join(&new_entry.name);

                        return Ok(Some(extracted_path));
                    }
                }
            } else if entry_path.is_dir()
                && let Some(result) = entry_path.find_file(name, extract_if_necessary)?
            {
                return Ok(Some(result));
            }
        }

        Ok(None)
    }

    fn has_extension(&self, extension: &str) -> bool {
        let path_extension = self.extension().and_then(OsStr::to_str);

        if let Some(path_extension) = path_extension
            && path_extension == extension
        {
            return true;
        }

        false
    }

    fn move_entry_inside(&self, entry: &DirEntry) -> Result<()> {
        let entry_path = entry.path();
        let file_name = entry_path
            .file_name()
            .with_context(|| format!("Couldn't get entry file name: {entry_path:?}"))?;
        let destination = self.join(file_name);

        rename(entry_path, destination)?;

        Ok(())
    }
}

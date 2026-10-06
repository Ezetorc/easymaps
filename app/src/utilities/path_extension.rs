use std::{
    ffi::OsStr,
    fs::{DirEntry, read_dir, remove_dir_all, remove_file, rename},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub trait PathExtension {
    fn clean_directory(&self) -> Result<()>;
    fn find_file(&self, file_name: &str) -> Result<Option<PathBuf>>;
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

    fn find_file(&self, file_name: &str) -> Result<Option<PathBuf>> {
        let entries = read_dir(self)?.collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let entry_path = entry.path();

            if entry_path.is_file() {
                if entry.file_name() == file_name {
                    return Ok(Some(entry_path));
                }
            } else if entry_path.is_dir() {
                let result = entry_path.find_file(file_name)?;

                match result {
                    Some(path) => return Ok(Some(path)),
                    None => continue,
                }
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

use anyhow::Result;
use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn find_file(root: &Path, file_name: &str) -> Result<Option<PathBuf>> {
    let entries = read_dir(root)?.collect::<Result<Vec<_>, _>>()?;

    for entry in entries {
        let entry_path = entry.path();

        if entry_path.is_file() {
            if entry.file_name() == file_name {
                return Ok(Some(entry_path));
            }
        } else if entry_path.is_dir() {
            let result = find_file(&entry_path, file_name)?;

            match result {
                Some(path) => return Ok(Some(path)),
                None => continue,
            }
        }
    }

    Ok(None)
}

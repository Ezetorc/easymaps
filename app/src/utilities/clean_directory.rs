use anyhow::Result;
use std::{
    fs::{read_dir, remove_dir_all, remove_file},
    path::Path,
};

pub fn clean_directory(path: &Path) -> Result<()> {
    let entries = read_dir(path)?.collect::<Result<Vec<_>, _>>()?;

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

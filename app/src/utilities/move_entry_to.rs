use std::{
    fs::{DirEntry, rename},
    path::Path,
};

use anyhow::{Context, Result};
pub fn move_entry_to(entry: &DirEntry, to: &Path) -> Result<()> {
    let entry_path = entry.path();
    let file_name = entry_path
        .file_name()
        .with_context(|| format!("Couldn't get entry file name: {entry_path:?}"))?;
    let destination = to.join(file_name);

    rename(entry_path, destination)?;

    Ok(())
}

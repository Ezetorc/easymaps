use std::{
    fs::{DirEntry, rename},
    path::Path,
};

use crate::errors::{app_error::AppError, app_io_error::AppIoError};

pub fn move_entry_to(entry: &DirEntry, to: &Path) -> Result<(), AppError> {
    let entry_path = entry.path();
    let file_name = entry_path.file_name().ok_or_else(|| {
        AppError::Io(AppIoError::NameError(format!(
            "Couldn't get entry file name: {entry_path:?}"
        )))
    })?;
    let destination = to.join(file_name);

    rename(entry_path, destination)?;

    Ok(())
}

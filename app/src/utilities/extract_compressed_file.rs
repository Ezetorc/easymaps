use std::path::{Path, PathBuf};

use stuffr::{
    entries::{ExtractOpts, Selection},
    ops::Input,
};

use crate::errors::{app_error::AppError, app_io_error::AppIoError};

pub fn extract_compressed_file(
    compressed_file: PathBuf,
    destination: &Path,
) -> Result<(), AppError> {
    stuffr::entries::extract(
        Input::Path(compressed_file),
        destination,
        &Selection::All,
        &ExtractOpts {
            compressed_total: None,
            memory_limit: None,
            max_ratio: 256,
            force: false,
        },
    )
    .map_err(|error| AppError::Io(AppIoError::ExtractionFailed(error.to_string())))?;

    Ok(())
}

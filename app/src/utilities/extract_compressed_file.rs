use std::path::{Path, PathBuf};

use stuffr::{
    entries::{ExtractOpts, Selection},
    ops::Input,
};

use anyhow::{Context, Result};

pub fn extract_compressed_file(compressed_file: PathBuf, destination: &Path) -> Result<()> {
    stuffr::entries::extract(
        Input::Path(compressed_file.clone()),
        destination,
        &Selection::All,
        &ExtractOpts {
            compressed_total: None,
            memory_limit: None,
            max_ratio: 256,
            force: false,
        },
    )
    .with_context(|| format!("Failed to extract compressed file with path '{compressed_file:?}"))?;

    Ok(())
}

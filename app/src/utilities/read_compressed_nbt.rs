use flate2::read::GzDecoder;
use std::{fmt::Debug, fs::File, io::Read, path::Path};

use anyhow::{Context, Result};
pub fn read_compressed_nbt<P: AsRef<Path> + Debug>(path: P) -> Result<Vec<u8>> {
    let file = File::open(&path).context("While reading compressed nbt")?;
    let mut gzip_decoder = GzDecoder::new(file);
    let mut nbt = Vec::new();

    gzip_decoder
        .read_to_end(&mut nbt)
        .with_context(|| format!("Reading path '{path:?}'"))?;

    Ok(nbt)
}

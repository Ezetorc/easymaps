use flate2::read::GzDecoder;
use std::{fs::File, io::Read, path::Path};

use crate::errors::app_error::AppError;

pub fn read_compressed_nbt<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, AppError> {
    let file = File::open(path)?;
    let mut gzip_decoder = GzDecoder::new(file);
    let mut nbt = Vec::new();

    gzip_decoder.read_to_end(&mut nbt)?;

    Ok(nbt)
}

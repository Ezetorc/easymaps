use std::{
    ffi::OsStr,
    fs::{DirEntry, create_dir, read_dir, remove_dir_all, remove_file, rename},
    path::{Path, PathBuf},
};

use stuffr::{
    entries::{ExtractOpts, Selection},
    ops::Input,
};

use crate::{
    app_paths::AppPaths,
    errors::{
        app_error::AppError, app_io_error::AppIoError, app_minecraft_error::AppMinecraftError,
    },
    messaging::{request::Request, response::Response},
    minecraft::{minecraft_launcher::MinecraftLauncher, world_version::WorldVersion},
};

mod app_paths;
mod errors;
mod messaging;
mod minecraft;
mod utilities;

fn main() {
    loop {
        match Request::read() {
            Ok(Some(request)) => {
                if let Err(error) = handle_request(request) {
                    let _ = Response::Error.send();
                    log!("❌ {error}");
                }
            }

            Ok(None) => {
                break;
            }

            Err(error) => {
                let _ = Response::Error.send();
                log!("❌ {error}");
            }
        }
    }
}

fn handle_request(request: Request) -> Result<(), AppError> {
    log!("[Request received] {request:?}");

    match request {
        Request::Start {
            minecraft_version,
            download_path,
        } => handle_start_request(minecraft_version, download_path),
    }
}

fn handle_start_request(minecraft_version: String, download_path: String) -> Result<(), AppError> {
    Response::Starting.send()?;

    let download_path = PathBuf::from(download_path);
    let temp_directory_path = AppPaths::temp()?;

    if download_path.is_dir() {
        rename(download_path, &temp_directory_path)?;
    } else if download_path.is_file() {
        extract_compressed_file(download_path, &temp_directory_path)?;
        normalize_entries(&temp_directory_path)?;
    }

    let level_dat_file_path = find_file(&temp_directory_path, "level.dat")?.ok_or_else(|| {
        AppError::Minecraft(AppMinecraftError::World(
            "Downloaded folder is not a Minecraft world".to_string(),
        ))
    })?;

    let world_directory_path = level_dat_file_path.parent().ok_or_else(|| {
        AppError::Io(AppIoError::NamingError(
            "Couldn't get file path's parent".to_string(),
        ))
    })?;
    let world_name = world_directory_path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| {
            AppError::Io(AppIoError::NamingError(
                "Couldn't get directory's file name".to_string(),
            ))
        })?;
    let final_path = AppPaths::world(world_name)?;

    rename(world_directory_path, &final_path)?;

    let version = WorldVersion::from_world(final_path)?.unwrap_or(minecraft_version);

    clean_directory(&temp_directory_path)?;

    let launch_result = MinecraftLauncher::launch(&version, world_name, None);

    match launch_result {
        Ok(_) => {
            Response::Finished.send()?;
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn find_file(root: &Path, file_name: &str) -> Result<Option<PathBuf>, AppError> {
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

fn clean_directory(path: &Path) -> Result<(), AppError> {
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

fn normalize_entries(directory_path: &Path) -> Result<(), AppError> {
    let directory_entries = read_dir(directory_path)?.collect::<Result<Vec<_>, _>>()?;
    let entries_count = directory_entries.len();

    match entries_count {
        0 => {
            return Err(AppError::Io(AppIoError::InvalidEntries(
                "Expected at least 1 entry inside downloaded file, found 0".to_string(),
            )));
        }
        1 => {
            let entry = &directory_entries[0];

            if entry.path().is_file() {
                let new_directory_path = &directory_path.join("Extracted");

                create_dir(new_directory_path)?;
                move_entry_to(entry, new_directory_path)?;
            }
        }
        2.. => {
            let new_directory_path = &directory_path.join("Extracted");

            create_dir(new_directory_path)?;
            move_entries_to(directory_entries, new_directory_path)?;
        }
    }

    Ok(())
}

fn extract_compressed_file(compressed_file: PathBuf, destination: &Path) -> Result<(), AppError> {
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

fn move_entry_to(entry: &DirEntry, to: &Path) -> Result<(), AppError> {
    let entry_path = entry.path();
    let file_name = entry_path.file_name().ok_or_else(|| {
        AppError::Io(AppIoError::NamingError(format!(
            "Couldn't get entry file name: {entry_path:?}"
        )))
    })?;
    let destination = to.join(file_name);

    rename(entry_path, destination)?;

    Ok(())
}

fn move_entries_to(entries: Vec<DirEntry>, to: &Path) -> Result<(), AppError> {
    for entry in entries {
        move_entry_to(&entry, to)?;
    }

    Ok(())
}

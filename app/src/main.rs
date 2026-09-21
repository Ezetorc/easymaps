use std::{fs, path::Path};

use stuffr::{
    entries::{ExtractOpts, Selection},
    ops::Input,
};

use crate::{
    app_paths::AppPaths,
    errors::{app_error::AppError, app_minecraft_error::AppMinecraftError},
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
                    log!("❌ {error}");
                }
            }

            Ok(None) => {
                break;
            }

            Err(error) => {
                log!("❌ {error}");
            }
        }
    }
}

fn handle_request(request: Request) -> Result<(), AppError> {
    log!("Request received: {request:?}");

    match request {
        Request::Start {
            minecraft_version,
            filename,
        } => handle_start_request(minecraft_version, filename),
    }
}

fn handle_start_request(minecraft_version: String, filename: String) -> Result<(), AppError> {
    Response::Starting.send()?;

    extract_world(&filename)?;

    let world_name: Option<String> = determine_world_name()?;
    let version = match world_name.as_deref() {
        Some(name) => WorldVersion::from_world(name)?,
        None => minecraft_version,
    };

    let launch_result = MinecraftLauncher::launch(&version, world_name.as_deref(), None);

    match launch_result {
        Ok(_) => Response::Finished.send()?,
        Err(error) => Response::Error {
            error: error.to_string(),
        }
        .send()?,
    }

    Ok(())
}

fn extract_world(filename: &String) -> Result<(), AppError> {
    stuffr::entries::extract(
        Input::Path(filename.into()),
        Path::new(&AppPaths::worlds()?),
        &Selection::All,
        &ExtractOpts {
            compressed_total: None,
            memory_limit: None,
            max_ratio: 256,
            force: true,
        },
    )
    .map_err(|error| {
        AppError::Minecraft(AppMinecraftError::World(format!(
            "Error while extracting world: {error}"
        )))
    })?;

    Ok(())
}

fn determine_world_name() -> Result<Option<String>, AppError> {
    let worlds_directory = AppPaths::worlds()?;
    let mut read_result = fs::read_dir(worlds_directory)?;

    Ok(read_result.find_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();

        if !path.is_dir() {
            return None;
        }

        let level_dat_path = path.join("level.dat");
        let level_dat_exists = level_dat_path.try_exists().unwrap_or(false);

        if level_dat_exists {
            let last_component = path.file_name();

            if let Some(directory_name) = last_component {
                return Some(directory_name.display().to_string());
            }
        }

        None
    }))
}

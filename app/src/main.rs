use anyhow::{Context, Result, bail};
use std::{
    fs::{remove_dir_all, remove_file},
    path::PathBuf,
};

use crate::{
    messaging::{
        request::Request,
        response::{AppError, Response},
    },
    minecraft::{
        minecraft_launcher::MinecraftLauncher, world::World, world_importer::WorldImporter,
    },
    utilities::{app_paths::AppPaths, path_extension::PathExtension},
};

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
                let _ = Response::ConnectionError.send();
                log!("❌ {error}");
            }
        }
    }
}

fn handle_request(request: Request) -> Result<()> {
    log!("[Request received] {request:?}");

    match request {
        Request::StartMap {
            minecraft_version,
            download_path,
            requester_id,
            minecraft_username,
        } => {
            let path = download_path.clone();

            if let Err(error) = handle_start_request(
                minecraft_version,
                minecraft_username,
                download_path,
                requester_id,
            ) {
                log!("❌ {error}");

                Response::AppError {
                    requester_id,
                    error: AppError::Unexpected,
                }
                .send()?;
            }

            remove_file(&path).with_context(|| format!("Removing file '{:?}'", path))?;

            Ok(())
        }
        Request::CheckMap {
            world_name,
            requester_id,
        } => handle_check_map_request(world_name, requester_id),
        Request::ContinueMap {
            world_name,
            requester_id,
            minecraft_version,
            minecraft_username,
        } => handle_continue_map_request(
            minecraft_version,
            minecraft_username,
            world_name,
            requester_id,
        ),
        Request::DeleteMap {
            world_name,
            requester_id,
        } => handle_delete_map_request(world_name, requester_id),
    }
}

fn handle_delete_map_request(world_name: String, requester_id: i32) -> Result<()> {
    Response::CheckingMap { requester_id }.send()?;

    let minecraft_worlds_folder = AppPaths::worlds()?;
    let world_exists = minecraft_worlds_folder
        .find_directory(&world_name)?
        .is_some();

    if world_exists {
        Response::DeletingMap { requester_id }.send()?;

        let world_path = AppPaths::world(world_name)?;
        let result = remove_dir_all(&world_path);

        match result {
            Ok(_) => Response::MapDeleteSuccess { requester_id }.send()?,
            Err(error) => {
                Response::AppError {
                    requester_id,
                    error: AppError::Unexpected,
                }
                .send()?;

                bail!(error)
            }
        }
    } else {
        Response::AppError {
            requester_id,
            error: AppError::WorldNotFound,
        }
        .send()?;
    }

    Ok(())
}

fn handle_continue_map_request(
    minecraft_version: Option<String>,
    minecraft_username: Option<String>,
    world_name: String,
    requester_id: i32,
) -> Result<()> {
    let world = World::new(&world_name, AppPaths::world(world_name.clone())?);
    let version = world.find_version()?.or(minecraft_version);

    match version {
        Some(version) => {
            Response::InstallingVersion { requester_id }.send()?;
            MinecraftLauncher::launch(&version, world.name(), minecraft_username.as_deref())?;
            Response::LaunchingMinecraft { requester_id }.send()?;
        }
        None => Response::AppError {
            requester_id,
            error: AppError::UnknownWorldVersion,
        }
        .send()?,
    }

    Ok(())
}

fn handle_check_map_request(world_name: String, requester_id: i32) -> Result<()> {
    Response::CheckingMap { requester_id }.send()?;

    let minecraft_worlds_folder = AppPaths::worlds()?;
    let world_exists = minecraft_worlds_folder
        .find_directory(&world_name)?
        .is_some();

    Response::MapCheckResult {
        requester_id,
        world_exists,
    }
    .send()?;

    Ok(())
}

fn handle_start_request(
    minecraft_version: Option<String>,
    minecraft_username: Option<String>,
    download_path: String,
    requester_id: i32,
) -> Result<()> {
    Response::ImportingMap { requester_id }.send()?;

    let download_path = PathBuf::from(download_path);

    if download_path.has_extension("mcworld") {
        Response::AppError {
            requester_id,
            error: AppError::BedrockWorldNotSupported,
        }
        .send()?;

        return Ok(());
    }

    let world = WorldImporter::import(&download_path)?;
    let version = world.find_version()?.or(minecraft_version);

    match version {
        Some(version) => {
            Response::InstallingVersion { requester_id }.send()?;
            MinecraftLauncher::launch(&version, world.name(), minecraft_username.as_deref())?;
            Response::LaunchingMinecraft { requester_id }.send()?;
        }
        None => Response::AppError {
            requester_id,
            error: AppError::UnknownWorldVersion,
        }
        .send()?,
    }

    Ok(())
}

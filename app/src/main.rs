use anyhow::{Context, Result};
use std::{fs::remove_file, path::PathBuf};

use crate::{
    messaging::{
        request::Request,
        response::{AppError, Response},
    },
    minecraft::{minecraft_launcher::MinecraftLauncher, world_importer::WorldImporter},
    utilities::path_extension::PathExtension,
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
        Request::Start {
            minecraft_version,
            download_path,
            requester_id,
        } => {
            let path = download_path.clone();

            if let Err(error) = handle_start_request(minecraft_version, download_path, requester_id)
            {
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
    }
}

fn handle_start_request(
    minecraft_version: Option<String>,
    download_path: String,
    requester_id: i32,
) -> Result<()> {
    Response::Importing { requester_id }.send()?;

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
            Response::Installing { requester_id }.send()?;
            MinecraftLauncher::launch(&version, world.name(), None)?;
            Response::Launching { requester_id }.send()?;
        }
        None => Response::AppError {
            requester_id,
            error: AppError::UnknownWorldVersion,
        }
        .send()?,
    }

    Ok(())
}

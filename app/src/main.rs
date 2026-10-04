use anyhow::Result;
use std::path::PathBuf;

use crate::{
    messaging::{request::Request, response::Response},
    minecraft::{minecraft_launcher::MinecraftLauncher, world_importer::WorldImporter},
};

mod messaging;
mod minecraft;
mod utilities;

fn main() {
    loop {
        match Request::read() {
            Ok(Some(request)) => handle_request(request),

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

fn handle_request(request: Request) {
    log!("[Request received] {request:?}");

    match request {
        Request::Start {
            minecraft_version,
            download_path,
            requester_id,
        } => {
            let result = handle_start_request(minecraft_version, download_path, requester_id);

            if let Err(error) = result {
                let _ = Response::AppError { requester_id }.send();
                log!("❌ {error}");
            }
        }
    }
}

fn handle_start_request(
    minecraft_version: String,
    download_path: String,
    requester_id: i32,
) -> Result<()> {
    Response::Importing { requester_id }.send()?;

    let download_path = PathBuf::from(download_path);
    let world = WorldImporter::import(download_path)?;
    let version = world.find_version()?.unwrap_or(minecraft_version);

    Response::Installing { requester_id }.send()?;

    let launch_result = MinecraftLauncher::launch(&version, world.name(), None);

    match launch_result {
        Ok(_) => {
            Response::Launching { requester_id }.send()?;
            Ok(())
        }
        Err(error) => Err(error),
    }
}

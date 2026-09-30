use std::path::PathBuf;

use crate::{
    errors::app_error::AppError,
    messaging::{request::Request, response::Response},
    minecraft::{minecraft_launcher::MinecraftLauncher, world_importer::WorldImporter},
};

mod app_paths;
mod errors;
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
            web_tab_id,
        } => {
            let result = handle_start_request(minecraft_version, download_path, web_tab_id);

            if let Err(error) = result {
                let _ = Response::AppError { web_tab_id }.send();
                log!("❌ {error}");
            }
        }
    }
}

fn handle_start_request(
    minecraft_version: String,
    download_path: String,
    web_tab_id: Option<i32>,
) -> Result<(), AppError> {
    Response::Starting { web_tab_id }.send()?;

    let download_path = PathBuf::from(download_path);
    let world = WorldImporter::import(download_path)?;
    let version = world.find_version()?.unwrap_or(minecraft_version);
    let launch_result = MinecraftLauncher::launch(&version, world.name(), None);

    match launch_result {
        Ok(_) => {
            Response::Finished { web_tab_id }.send()?;
            Ok(())
        }
        Err(error) => Err(error),
    }
}

// Add account name option in extension
// Changeb utton style and loading

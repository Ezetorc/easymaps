use crate::{
    minecraft::minecraft_launcher::MinecraftLauncher,
    native_messaging::{request::Request, response::Response},
    utilities::log,
};

mod app_error;
mod extractor;
mod minecraft;
mod native_messaging;
mod utilities;

fn main() {
    while let Some(request) = Request::get() {
        match request {
            Request::Start {
                minecraft_version,
                filename,
            } => {
                log(&format!("[Request]: {minecraft_version} # {filename}"));

                if let Err(error) = &Response::Starting.send() {
                    log(&format!("[Error sending 'Starting' response] {error}"));
                    break;
                }

                let result = MinecraftLauncher::open_world(
                    "Mi Mapa de Aventuras",
                    "EasyMaps",
                    minecraft_version,
                );

                match result {
                    Ok(_) => log("[EasyMaps] Success"),
                    Err(error) => log(&format!("[EasyMaps] Error: {error:?}")),
                }

                if let Err(error) = &Response::Finished.send() {
                    log(&format!("[Error sending 'Finished' response] {error}"));
                    break;
                }
            }
        }
    }
}

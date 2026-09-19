use std::{
    env, fs,
    path::{Path, PathBuf},
};

use stuffr::{
    entries::{ExtractOpts, Selection},
    ops::Input,
};

use crate::{
    minecraft::minecraft_launcher::MinecraftLauncher,
    native_messaging::{request::Request, response::Response},
};

mod app_error;
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
                log!("[Request]: {minecraft_version} # {filename}");

                if let Err(error) = &Response::Starting.send() {
                    log!("[Error sending 'Starting' response] {error}");
                    break;
                }

                let Some(local_app_data) = env::var_os("LOCALAPPDATA") else {
                    log!("Error obtaining LocalAppData path");
                    break;
                };

                let easy_maps_folder = PathBuf::from(local_app_data).join("EasyMaps");
                let easy_maps_minecraft_folder = easy_maps_folder.join("Minecraft");
                let worlds_folder = easy_maps_minecraft_folder.join("saves");

                let extract_result = stuffr::entries::extract(
                    Input::Path(filename.into()),
                    Path::new(&worlds_folder),
                    &Selection::All,
                    &ExtractOpts {
                        compressed_total: None,
                        force: true,
                        max_ratio: 256,
                        memory_limit: None,
                    },
                );

                match extract_result {
                    Ok(output) => log!("Success: {:?}", output),
                    Err(error) => log!("Error: {}", error),
                }

                let read_result = fs::read_dir(&worlds_folder);
                let mut world_directory_name: Option<String> = None;

                match read_result {
                    Ok(output) => {
                        for entry in output {
                            match entry {
                                Ok(entry) => {
                                    log!("[Entry] Success: {entry:?}");
                                    let path = entry.path();

                                    if path.is_dir() {
                                        let level_dat_path = entry.path().join("level.dat");
                                        let level_dat_exists =
                                            level_dat_path.try_exists().unwrap_or(false);

                                        if level_dat_exists {
                                            let last_component = path.iter().next_back();

                                            if let Some(directory_name) = last_component {
                                                world_directory_name =
                                                    Some(directory_name.display().to_string());

                                                log!("[Directory Name] {world_directory_name:?}");
                                                break;
                                            }
                                        }
                                    }
                                }
                                Err(error) => log!("[Entry] Error: {error:?}"),
                            }
                        }
                    }
                    Err(error) => log!("[Read] Error: {error:?}"),
                }

                let launch_result = MinecraftLauncher::open_world(
                    world_directory_name,
                    "EasyMaps",
                    minecraft_version,
                );

                match launch_result {
                    Ok(_) => log!("[EasyMaps] Success"),
                    Err(error) => log!("[EasyMaps] Error: {error:?}"),
                }

                if let Err(error) = &Response::Finished.send() {
                    log!("[Error sending 'Finished' response] {error}");
                    break;
                }
            }
        }
    }
}

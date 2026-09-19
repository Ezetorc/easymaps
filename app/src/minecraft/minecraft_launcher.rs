use std::{env, path::PathBuf};

use crate::{
    app_error::AppError,
    minecraft::{minecraft_launch::MinecraftLaunch, world_version::WorldVersion},
};

pub struct MinecraftLauncher;

impl MinecraftLauncher {
    pub fn open_world(
        world_name: &str,
        account_name: &str,
        version_fallback: String,
    ) -> Result<(), AppError> {
        let local_app_data = env::var_os("LOCALAPPDATA").ok_or(AppError::NotFound(
            "Local AppData folder not found".to_string(),
        ))?;
        let easy_maps_folder = PathBuf::from(local_app_data).join("EasyMaps");
        let easy_maps_minecraft_folder = easy_maps_folder.join("Minecraft");
        let worlds_folder = easy_maps_minecraft_folder.join("saves");
        let world_version =
            WorldVersion::from_world(&worlds_folder.join(world_name), version_fallback)?;

        MinecraftLaunch::on_version(world_version.version())
            .game_directory(easy_maps_minecraft_folder)
            .on_world(world_name)
            .account_name(account_name)
            .start()?;

        Ok(())
    }
}

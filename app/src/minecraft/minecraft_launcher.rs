use std::{env, path::PathBuf};

use crate::{
    app_error::AppError,
    minecraft::{minecraft_launch::MinecraftLaunch, world_version::WorldVersion},
};

pub struct MinecraftLauncher;

impl MinecraftLauncher {
    pub fn open_world(
        world_name: Option<String>,
        account_name: &str,
        version_fallback: String,
    ) -> Result<(), AppError> {
        let local_app_data = env::var_os("LOCALAPPDATA").ok_or(AppError::NotFound(
            "Local AppData folder not found".to_string(),
        ))?;
        let easy_maps_folder = PathBuf::from(local_app_data).join("EasyMaps");
        let easy_maps_minecraft_folder = easy_maps_folder.join("Minecraft");
        let worlds_folder = easy_maps_minecraft_folder.join("saves");

        let mut binding = MinecraftLaunch::new(&version_fallback);
        let minecraft_launch = binding
            .game_directory(easy_maps_minecraft_folder)
            .account_name(account_name);

        if let Some(world_name) = world_name {
            let world_version =
                WorldVersion::from_world(&worlds_folder.join(&world_name), version_fallback)?;

            minecraft_launch
                .version(world_version.version().to_string())
                .world_name(&world_name);
        }

        minecraft_launch.start()?;

        Ok(())
    }
}

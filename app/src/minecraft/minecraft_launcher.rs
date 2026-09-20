use std::process::Command;

use mc_launcher_core::{
    account::Account, command::builder::LaunchOptions, install::InstallRequest, launcher::Launcher,
    utils::get_minecraft_directory,
};

use crate::{
    app_paths::AppPaths,
    errors::{app_error::AppError, app_minecraft_error::AppMinecraftError},
};

pub struct MinecraftLauncher;

impl MinecraftLauncher {
    pub fn launch(
        version: &String,
        world_name: Option<String>,
        account_name: Option<String>,
    ) -> Result<(), AppError> {
        let launcher = Launcher::new(get_minecraft_directory());
        let install_result = launcher
            .install(InstallRequest::vanilla(version.to_owned()))
            .map_err(|error| AppError::Minecraft(AppMinecraftError::Launcher(error)))?;
        let version = launcher
            .load_version(&install_result.version_id)
            .map_err(|error| AppError::Minecraft(AppMinecraftError::Launcher(error)))?;
        let account_name = account_name.unwrap_or(String::from("EasyMaps"));

        let options = LaunchOptions {
            account: Account::offline(account_name),
            game_directory: Some(AppPaths::minecraft()?),
            ..Default::default()
        };
        let launch_command = launcher
            .build_launch_command_from_version(&version, options)
            .map_err(|error| AppError::Minecraft(AppMinecraftError::Launcher(error)))?;

        let mut command = Command::new(&launch_command.executable);
        command
            .args(&launch_command.args)
            .current_dir(&launch_command.working_dir);

        if let Some(world_name) = world_name {
            command.args(["--quickPlaySingleplayer", world_name.as_str()]);
        }

        command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        Ok(())
    }
}

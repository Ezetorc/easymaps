use std::process::Command;

use anyhow::Result;
use mc_launcher_core::{
    account::Account,
    command::builder::{LaunchCommand, LaunchOptions},
    core::version::VersionJson,
    install::InstallRequest,
    launcher::Launcher,
    utils::get_minecraft_directory,
};

use crate::utilities::app_paths::AppPaths;

pub struct MinecraftLauncher;

impl MinecraftLauncher {
    pub fn launch(version: &str, world_name: &str, account_name: Option<&str>) -> Result<()> {
        let minecraft_directory = get_minecraft_directory();
        let launcher = Launcher::new(minecraft_directory);
        let version = Self::load_version(&launcher, version)?;
        let launch_command = Self::create_launch_command(&launcher, version, account_name)?;

        Self::execute_launch_command(&launch_command, world_name)?;

        Ok(())
    }

    fn load_version(launcher: &Launcher, version: &str) -> Result<VersionJson> {
        let install_result = launcher.install(InstallRequest::vanilla(version))?;

        let version: mc_launcher_core::core::version::VersionJson =
            launcher.load_version(&install_result.version_id)?;

        Ok(version)
    }

    fn create_launch_command(
        launcher: &Launcher,
        version: VersionJson,
        account_name: Option<&str>,
    ) -> Result<LaunchCommand> {
        let account_name = account_name.unwrap_or("EasyMaps");

        let options = LaunchOptions {
            account: Account::offline(account_name),
            game_directory: Some(AppPaths::minecraft()?),
            ..Default::default()
        };

        let launch_command = launcher.build_launch_command_from_version(&version, options)?;

        Ok(launch_command)
    }

    fn execute_launch_command(launch_command: &LaunchCommand, world_name: &str) -> Result<()> {
        let mut command = Command::new(&launch_command.executable);

        command
            .args(&launch_command.args)
            .args(["--quickPlaySingleplayer", world_name])
            .current_dir(&launch_command.working_dir);

        command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        Ok(())
    }
}

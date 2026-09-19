use mc_launcher_core::{
    account::Account, command::builder::LaunchOptions, install::InstallRequest, launcher::Launcher,
    utils::get_minecraft_directory,
};
use std::{path::PathBuf, process::Command};

use crate::app_error::AppError;

pub struct MinecraftLaunch {
    version: String,
    game_directory: Option<PathBuf>,
    world_name: Option<String>,
    account_name: String,
}

impl MinecraftLaunch {
    pub fn new(version: &String) -> Self {
        Self {
            version: version.to_owned(),
            account_name: String::from("Steve"),
            game_directory: None,
            world_name: None,
        }
    }

    pub fn version(&mut self, version: String) -> &mut Self {
        self.version = version;
        self
    }

    pub fn game_directory(&mut self, game_directory: PathBuf) -> &mut Self {
        self.game_directory = Some(game_directory);
        self
    }

    pub fn world_name(&mut self, world_name: &String) -> &mut Self {
        self.world_name = Some(world_name.to_string());
        self
    }

    pub fn account_name(&mut self, account_name: &str) -> &mut Self {
        self.account_name = account_name.to_string();
        self
    }

    pub fn start(&self) -> Result<(), AppError> {
        let launcher = Launcher::new(get_minecraft_directory());
        let install_result = launcher.install(InstallRequest::vanilla(self.version.to_owned()))?;
        let version = launcher.load_version(&install_result.version_id)?;

        let options = LaunchOptions {
            account: Account::offline(self.account_name.clone()),
            game_directory: self.game_directory.to_owned(),
            ..Default::default()
        };
        let launch_command = launcher.build_launch_command_from_version(&version, options)?;

        let mut command = Command::new(&launch_command.executable);
        command
            .args(&launch_command.args)
            .current_dir(&launch_command.working_dir);

        if let Some(world_name) = &self.world_name {
            command.args(["--quickPlaySingleplayer", world_name.as_str()]);
        }

        command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        Ok(())
    }
}

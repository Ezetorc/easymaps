use mc_launcher_core::{
    account::Account, command::builder::LaunchOptions, install::InstallRequest, launcher::Launcher,
    utils::get_minecraft_directory,
};
use std::{env, path::PathBuf, process::Command};

use crate::{app_error::AppError, world_version::WorldVersion};

mod app_error;
mod easymaps_config;
mod world_version;

const ACCOUNT_NAME: &str = "Ezetorx";
const WORLD_NAME: &str = "Mi Mapa de Aventurasc";

fn main() {
    if let Err(error) = run() {
        println!("[ERROR] {error}")
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let local_app_data = env::var_os("LOCALAPPDATA").ok_or(AppError::NotFound(
        "Local AppData folder not found".to_string(),
    ))?;
    let easy_maps_folder = PathBuf::from(local_app_data).join("EasyMaps");
    let worlds_folder = easy_maps_folder.join("saves");
    let world_version = WorldVersion::from_world(&worlds_folder.join(WORLD_NAME))?;

    let launcher = Launcher::new(get_minecraft_directory());
    let install_result = launcher.install(InstallRequest::vanilla(world_version.version()))?;
    let version = launcher.load_version(&install_result.version_id)?;

    let options = LaunchOptions {
        account: Account::offline(ACCOUNT_NAME),
        game_directory: Some(easy_maps_folder),
        ..Default::default()
    };
    let launch_command = launcher.build_launch_command_from_version(&version, options)?;

    let mut child = Command::new(&launch_command.executable)
        .args(&launch_command.args)
        .args(["--quickPlaySingleplayer", WORLD_NAME])
        .current_dir(&launch_command.working_dir)
        .spawn()?;
    let exit_status = child.wait()?;

    println!("[EXIT_STATUS] {exit_status}");

    Ok(())
}

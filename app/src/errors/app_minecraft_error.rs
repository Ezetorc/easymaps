use std::fmt::Display;

use mc_launcher_core::LauncherError;

#[derive(Debug)]
pub enum AppMinecraftError {
    Nbt(String),
    World(String),
    Launcher(LauncherError),
}

impl Display for AppMinecraftError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nbt(message) => write!(f, "NBT error: {message}"),
            Self::World(message) => write!(f, "World error: {message}"),
            Self::Launcher(error) => write!(f, "Launcher error: {error}"),
        }
    }
}

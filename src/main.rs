use crate::world_version::WorldVersion;

mod app_error;
mod world_version;

fn main() {
    let world_info = WorldVersion::from_level_dat("Mi Mapa de Aventuras/level.dat");

    match world_info {
        Ok(value) => println!("value: {value:?}"),
        Err(error) => println!("error: {error}"),
    }
}

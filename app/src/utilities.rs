use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn log(message: &str) {
    let Some(local_app_data) = env::var_os("LOCALAPPDATA") else {
        log("Error obtaining LocalAppData path");
        panic!("LocalAppData not found");
    };

    let path = PathBuf::from(local_app_data)
        .join("EasyMaps")
        .join("EasyMaps.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Couldn't write log");

    writeln!(file, "{message}").expect("Couldn't write log");
}

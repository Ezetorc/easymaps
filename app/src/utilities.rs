use std::{
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::app_paths::AppPaths;

pub fn write_log(args: std::fmt::Arguments) {
    let path = AppPaths::root().unwrap().join("EasyMaps.log");

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Couldn't create log file");

    writeln!(file, "{args}").expect("Couldn't write log");
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::utilities::write_log(format_args!($($arg)*))
    };
}

pub fn random_id() -> String {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH);

    match timestamp {
        Ok(duration) => {
            let stamp = duration.as_micros();
            let pid = std::process::id();

            format!("{stamp}_{pid}")
        }
        Err(_) => String::from("0"),
    }
}

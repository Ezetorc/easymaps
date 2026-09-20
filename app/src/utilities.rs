use std::io::Write;

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

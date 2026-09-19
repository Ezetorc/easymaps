use std::io::Write;

pub fn write_log(args: std::fmt::Arguments) {
    let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") else {
        panic!("LocalAppData not found");
    };

    let path = std::path::PathBuf::from(local_app_data)
        .join("EasyMaps")
        .join("EasyMaps.log");

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

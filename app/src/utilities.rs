use std::fs::OpenOptions;
use std::io::Write;

pub fn log(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")
        .expect("Couldn't write log");

    writeln!(file, "{message}").expect("Couldn't write log");
}

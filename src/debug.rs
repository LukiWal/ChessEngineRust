use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_PATH: &str = "/Users/luki/git/RustChessEngine/uci_debug.log";

pub fn log_debug(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    writeln!(file, "{}", message).unwrap();
}

pub fn log_value<T: Debug>(label: &str, value: &T) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    writeln!(file, "{}: {:#?}", label, value).unwrap();
}
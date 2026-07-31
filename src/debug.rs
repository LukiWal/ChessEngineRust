use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

#[cfg(target_os = "windows")]
const LOG_PATH: &str = r"C:\Users\LukasWalter\git\ChessEngineRust\uci_debug.log";

#[cfg(any(target_os = "macos", target_os = "linux"))]
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


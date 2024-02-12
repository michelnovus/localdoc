// [MIT License] Copyright (c) 2024 Michel Novus

extern crate lodolib;
use lodoctl::print;
// use lodolib::{api, process::RuntimeDir, socket};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app_path = PathBuf::from(&args[0]);
    match args.get(1) {
        Some(value_version) if value_version == &"--version".to_string() => {
            print::version(&app_path)
        }
        Some(value_unknown) => {
            print::unknown_cmd(value_unknown);
            print::help(&app_path);
        }
        None => print::help(&app_path),
    };
}

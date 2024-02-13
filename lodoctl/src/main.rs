// [MIT License] Copyright (c) 2024 Michel Novus

extern crate lodolib;
use fork;
use lodoctl::print;
use lodolib::{
    constants::{ARG_START, ARG_STOP},
    process::RuntimeDir,
    utils::stop_service,
};
use std::env;
use std::path::PathBuf;
use std::process;

/// Contiene solo los valores de `release` o `debug`.
const PROFILE: &'static str = env!("PROFILE_TYPE");

fn main() {
    let args: Vec<String> = env::args().collect();
    let app_path = PathBuf::from(&args[0]);
    match args.get(1) {
        Some(value_version) if value_version == &"--version".to_string() => {
            print::version(&app_path)
        }
        Some(value_help) if value_help == &"--help".to_string() => {
            print::help(&app_path)
        }
        Some(value_start) if value_start == ARG_START => {
            let runtime_dir = RuntimeDir::new().unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
            if runtime_dir.exists() {
                println!(
                    "Parece que ya se está ejecutando el servicio
                    `lodosrv`, cancelando..."
                );
                process::exit(1);
            } else {
                println!("Iniciando servicio...");
                if PROFILE == "debug" {
                    //NOTE: el primer argumento de fork::daemon cuando
                    // es false cambia la ruta del programa hijo a la raíz.
                    if let Ok(fork::Fork::Child) = fork::daemon(false, true) {
                        process::Command::new("/usr/bin/echo")
                            .arg("Implementar lodosrv.")
                            .spawn()
                            .expect("Error en bifurcar.");
                    }
                }
            }
        }
        Some(value_stop) if value_stop == ARG_STOP => {
            let runtime_dir = RuntimeDir::new().unwrap_or_else(|err| {
                println!("{}", err);
                process::exit(1);
            });
            stop_service(&runtime_dir).unwrap_or_else(|err| {
                println!(
                    "No se pudo detener el servicio, \
                    quizás no exista. ERROR: [{:?}]",
                    err.kind()
                );
                process::exit(1);
            });
            println!("Servicio detenido con éxito!");
        }
        Some(value_unknown) => {
            print::unknown_cmd(value_unknown);
            print::help(&app_path);
        }
        None => print::help(&app_path),
    };
}

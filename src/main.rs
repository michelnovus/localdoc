// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;

use localdoc_service::{
    service::{
        constants::*,
        process::{resolve_root_directory, RuntimeDir},
        Service,
    },
    utils::{self, generate_unit_service},
};
use std::{
    env,
    fs::{self, canonicalize},
    io::{self, prelude::*},
    path::PathBuf,
    process,
};
use users::get_current_username;

fn main() {
    let runtime_dir = RuntimeDir::new({
        &env::var(ROOT_DIRECTORY_ENV_VAR).unwrap_or_else(|_| {
            resolve_root_directory().unwrap_or_else(|| {
                eprintln!(
                    "No se pudo resolver el directorio raíz \
                    para el proceso!"
                );
                process::exit(1);
            })
        })
    });

    {
        let args: Vec<String> = env::args().collect();
        match args.get(1) {
            Some(value) if value == &String::from(ARG_STOP) => {
                match utils::stop_service(&runtime_dir) {
                    Ok(()) => {
                        println!("El servicio se detuvo con éxito.");
                        process::exit(0);
                    }
                    Err(err) => {
                        eprintln!(
                            "Error al intentar terminar el servicio: \
                    ERROR [{}]",
                            err.kind()
                        );
                        process::exit(1);
                    }
                }
            }
            Some(value) if value == &String::from(ARG_GENERATE) => {
                println!("Instalando servicio.");
                let username = get_current_username().unwrap_or_else(|| {
                    eprintln!("No se pudo obtener el nombre de usuario!");
                    process::exit(1);
                });
                let mut systemd_user_path =
                    String::from(SYSTEMD_USER_DIRECTORY);
                systemd_user_path
                    .replace_range(6..=13, &username.into_string().unwrap());
                let systemd_user_path = PathBuf::from(systemd_user_path);
                if !systemd_user_path.exists() {
                    fs::create_dir_all(&systemd_user_path).unwrap_or_default();
                }
                let mut systemd_unit = PathBuf::from(systemd_user_path);
                systemd_unit.push("localdoc-service.service");
                if !systemd_unit.exists() {
                    let mut file = fs::File::create(&systemd_unit).unwrap();
                    let file_content = generate_unit_service(
                        canonicalize(PathBuf::from(&args[0])).unwrap(),
                    );
                    file.write_all(file_content.as_bytes()).unwrap();
                    let mut cmd = process::Command::new("/usr/bin/systemctl")
                        .arg("--user")
                        .arg("daemon-reload")
                        .spawn()
                        .unwrap_or_else(|err| {
                            eprintln!(
                                "No se pudo recargar systemd, error [{}]",
                                err.kind()
                            );
                            process::exit(1);
                        });
                    cmd.wait().expect("Error al esperar a systemd.");
                } else {
                    eprintln!(
                        r#"El archivo "{}" ya existe."#,
                        systemd_unit.to_string_lossy()
                    );
                    process::exit(1);
                };
                println!(
                    r#"Archivo unidad instalado en: "{}""#,
                    systemd_unit.to_string_lossy()
                );
                process::exit(0);
            }
            Some(value) if value == &String::from(ARG_START) => {
                println!("Iniciando servicio...");
            }
            Some(value) => {
                eprintln!(r#"El argumento: "{}" no se reconoce."#, value);
                process::exit(1);
            }
            None => {
                eprintln!("No se pasaron argumentos");
                process::exit(1);
            }
        }
    };

    // Temporalmente no se permite ejecuar el proceso en subdirectorios
    // de HOME por probable eliminación de archivos del usuario...
    if runtime_dir.get_root().unwrap().starts_with("/home") {
        eprintln!(
            "El servicio no se puede ejecutar en los \
            directorios de usuario!"
        );
        process::exit(1);
    };
    match runtime_dir.make() {
        Ok(_) => {
            println!("Se creó el directorio: {:?}", runtime_dir.get_root());
        }
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!(
                "No tienes permisos para crear: {:?}",
                runtime_dir.get_root()
            );
            process::exit(1);
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            eprintln!("El directorio: {:?} ya existe", runtime_dir.get_root());
            process::exit(1);
        }
        Err(err) => panic!("Error indeterminado: {:#?}", err),
    };

    let mut service = Service::new(runtime_dir);
    match service.start() {
        Ok(()) => println!("Servicio terminado con éxito!"),
        Err(_) => println!("Error calamitoso, saliendo..."),
    };
}

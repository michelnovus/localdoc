// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;

use localdoc_service::{
    service::{
        constants::*,
        process::{resolve_root_directory, RuntimeDir},
        Service,
    },
    utils::{self, unit_service},
};
use std::{env, fs::canonicalize, io, path::PathBuf, process};

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

    let mut fork_process = false;

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
                            "No se pudo detener el servicio, es probable \
                            que no se estuviera ejecutando.\n\
                            ERROR: [{:?}]",
                            err.kind()
                        );
                        process::exit(0);
                    }
                }
            }
            Some(value) if value == &String::from(ARG_GENERATE) => {
                println!("Instalando servicio.");
                match unit_service::generate_unit_service(
                    canonicalize(PathBuf::from(&args[0])).unwrap(),
                ) {
                    Ok(()) => (),
                    Err(unit_service::Error::GetUsernameError) => {
                        eprintln!("No se pudo obtener el nombre de usuario!");
                        process::exit(1);
                    }
                    Err(unit_service::Error::CreateDirError) => {
                        eprintln!(
                            "No se pudo crear el direcotorio \
                            de unidades de systemd del usuario!"
                        );
                        process::exit(1);
                    }
                    Err(unit_service::Error::CreateUnitError) => {
                        eprintln!(
                            "Algo salió mal en la crearción de la unidad \
                            de systemd."
                        );
                        process::exit(1);
                    }
                    Err(unit_service::Error::UnitAlreadyExistsError) => {
                        eprintln!(
                            "La unidad localdoc-service.service ya existe."
                        );
                        process::exit(1);
                    }
                    Err(_) => (),
                };
                println!(
                    r#"Archivo unidad "localdoc-service.service" instalado"#
                );
                println!(
                    "Intentando recargar unidades del usuario de systemd."
                );
                if let Err(unit_service::Error::SystemdReloadError) =
                    unit_service::reload_systemd_units()
                {
                    eprintln!("Error al intentar recargar las unidades.");
                    process::exit(1);
                };
                println!("Las unidades se recargaron con éxito!");
                process::exit(0);
            }
            Some(value) if value == &String::from(ARG_START) => {
                if args.get(2) == Some(&"--fork".to_string()) {
                    fork_process = true;
                };
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
            println!(
                "Se creó el directorio: {:?}",
                runtime_dir.get_root().unwrap()
            );
        }
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!(
                "No tienes permisos para crear: {:?}",
                runtime_dir.get_root()
            );
            process::exit(1);
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            eprintln!(
                "El directorio: {:?} ya existe.\nParece que se está \
                ejecutando otra instancia del programa.",
                runtime_dir.get_root().unwrap()
            );
            process::exit(1);
        }
        Err(err) => panic!("Error indeterminado: {:#?}", err),
    };

    if fork_process {
        if let Ok(fork::Fork::Child) = fork::daemon(false, false) {
            process::Command::new(env::current_exe().unwrap())
                .arg("start")
                .output()
                .expect("Error en bifurcar.");
        }
    } else {
        println!("Iniciando servicio...");
        let mut service = Service::new(runtime_dir);
        match service.start() {
            Ok(()) => println!("Servicio terminado con éxito!"),
            Err(_) => println!("Error calamitoso, saliendo..."),
        };
    }
}

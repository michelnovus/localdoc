// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;

use localdoc_service::{
    service::{
        constants::*,
        process::{resolve_root_directory, RuntimeDir},
        Service,
    },
    utils,
};
use std::{env, io, process};

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
            Some(value) if value == &String::from("stop") => {
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
            Some(value) if value == &String::from("start") => {
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

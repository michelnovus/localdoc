// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;

use localdoc_service::service::{
    constants::*,
    process::{resolve_root_directory, RuntimeDir},
    Service,
};
use std::{env, io, process};

fn main() {
    let runtime_dir = {
        let root_dir = {
            env::var(ROOT_DIRECTORY_ENV_VAR).unwrap_or_else(|_| {
                resolve_root_directory().unwrap_or_else(|| {
                    eprintln!(
                        "No se pudo resolver el directorio raíz
                        para el proceso!"
                    );
                    process::exit(1);
                })
            })
        };
        let socket_name = env::var(SOCKET_NAME_ENV_VAR)
            .unwrap_or_else(|_| SOCKET_NAME_DEFAULT.to_string());
        RuntimeDir::new(&root_dir, &socket_name)
    };
    // Temporalmente no se permite ejecuar el proceso en subdirectorios
    // de HOME por probable eliminación de archivos del usuario...
    if runtime_dir.get_root().unwrap().starts_with("/home") {
        eprintln!(
            "El servicio no se puede ejecutar en los
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

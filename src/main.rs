// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;
use localdoc_service::{resolve_root_directory, RuntimeDir};
use std::env::var;
use std::io::ErrorKind;
use std::process;

fn main() {
    let runtime_directory = {
        let root_directory = resolve_root_directory("LOCALDOC_ROOT")
            .unwrap_or_else(|| process::exit(1));
        let socket_name = var("LOCALDOC_SOCKET")
            .unwrap_or_else(|_| String::from("localdoc.socket"));
        RuntimeDir::new(&root_directory, &socket_name)
    };
    match runtime_directory.create_root() {
        Ok(_) => println!(
            "Se creó el directorio: {:?}",
            runtime_directory.get_root()
        ),
        Err(err) if err.kind() == ErrorKind::PermissionDenied => {
            eprintln!(
                "No tienes permisos para crear: {:?}",
                runtime_directory.get_root()
            );
            process::exit(1);
        }
        Err(err) if err.kind() == ErrorKind::AlreadyExists => {
            eprintln!(
                "El directorio: {:?} ya existe",
                runtime_directory.get_root(),
            );
            process::exit(1); // Salir por seguridad de los datos en disco
        }
        Err(_) => panic!("Error indeterminado"),
    };
}

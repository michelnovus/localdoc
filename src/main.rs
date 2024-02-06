// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;
use localdoc_service::constants::*;
use localdoc_service::process::{
    resolve_root_directory, stop_process, RuntimeDir,
};
use localdoc_service::service::Service;
use localdoc_service::socket;
use std::env;
use std::io;
use std::os::unix::net::UnixListener;
use std::process;

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
    if runtime_dir.get_root().starts_with("/home") {
        eprintln!(
            "El servicio no se puede ejecutar en los
            directorios de usuario!"
        );
        process::exit(1);
    };
    match runtime_dir.create_root() {
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

    let listener =
        UnixListener::bind(runtime_dir.get_socket()).unwrap_or_else(|err| {
            eprint!("No se pudo crear el socket, error: [{:?}]", err.kind());
            process::exit(1);
        });
    println!(
        "Socket: {} iniciado.",
        runtime_dir.get_socket().to_str().unwrap()
    );
    let mut service = Service::new(runtime_dir);
    for conn in listener.incoming() {
        let mut stream = match conn {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("Error en la conexión: [{:?}]", err.kind());
                continue;
            }
        };

        let command = match socket::recv(&mut stream) {
            Ok(command) => command,
            Err(err) => {
                eprintln!(
                    "Error durante la lectura del flujo: [{:?}]",
                    err.kind()
                );
                continue;
            }
        };

        let stop_iteration = match service.execute_command(command) {
            Ok(resp) => {
                let stop_iteration = stop_process(&resp);
                socket::reply(&mut stream, resp).unwrap_or_else(|err| {
                    eprintln!("Error en la respuesta [{:?}]", err);
                });
                stop_iteration
            }
            Err(err) => {
                eprintln!("{:?}", err.kind());
                continue;
            }
        };

        if stop_iteration {
            println!("Servicio terminado");
            break;
        }
    }
}

// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;
use localdoc_service::socket;
use localdoc_service::socket::api::Status::{Failed, Success};
use localdoc_service::socket::api::{Command, Response};
use localdoc_service::start::{resolve_root_directory, RuntimeDir};
use std::env;
use std::io;
use std::matches;
use std::os::unix::net::UnixListener;
use std::process;

fn main() {
    let runtime_directory = {
        let root_directory = resolve_root_directory("LOCALDOC_ROOT")
            .unwrap_or_else(|| process::exit(1));
        let socket_name = env::var("LOCALDOC_SOCKET")
            .unwrap_or_else(|_| String::from("localdoc.socket"));
        RuntimeDir::new(&root_directory, &socket_name)
    };
    match runtime_directory.create_root() {
        Ok(_) => println!(
            "Se creó el directorio: {:?}",
            runtime_directory.get_root()
        ),
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            eprintln!(
                "No tienes permisos para crear: {:?}",
                runtime_directory.get_root()
            );
            process::exit(1);
        }
        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            eprintln!(
                "El directorio: {:?} ya existe",
                runtime_directory.get_root(),
            );
            process::exit(1); // Salir por seguridad de los datos en disco
        }
        Err(_) => panic!("Error indeterminado"),
    };

    let listener = UnixListener::bind(runtime_directory.get_socket())
        .unwrap_or_else(|err| {
            eprint!("No se pudo crear el socket, error: [{:?}]", err.kind());
            process::exit(1);
        });
    println!(
        "Socket: {} iniciado.",
        runtime_directory.get_socket().to_str().unwrap()
    );
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

        //DEBUG
        if matches!(command, Command::EXIT) {
            socket::reply(&mut stream, Response::EXIT(Success)).unwrap();
            break;
        }

        println!("{:?}", command);
        socket::reply(&mut stream, Response::STATUS { status: Success })
            .unwrap();
    }
}

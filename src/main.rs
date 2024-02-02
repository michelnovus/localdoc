// [MIT License] Copyright (c) 2024 Michel Novus

extern crate localdoc_service;
use localdoc_service::{resolve_root_directory, RuntimeDir};
use std::env;
use std::io;
use std::net::Shutdown;
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
        match conn {
            Ok(stream) => {
                println!("conectado y adiós");
                stream
                    .shutdown(Shutdown::Both)
                    .expect("No se pudo apagar la conexión!");
                break;
            }
            Err(err) => eprint!("Error en conexión entrante [{}]", err.kind()),
        };
    }
}

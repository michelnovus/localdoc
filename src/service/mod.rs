// [MIT License] Copyright (c) 2024 Michel Novus

pub mod constants;
pub mod error;
pub mod process;
pub mod socket;

use error::{ServiceError, ServiceErrorKind};
use process::{stop_process, RuntimeDir};
use socket::api::{
    Command, Command::*, DocKind::*, Package, Response, Status::*,
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    os::unix::net::UnixListener,
};

pub struct Service {
    runtime_dir: RuntimeDir,
}

impl Service {
    pub fn new(runtime_dir: RuntimeDir) -> Service {
        Service { runtime_dir }
    }

    fn make_socket(&self) -> Result<UnixListener, error::ServiceError> {
        match UnixListener::bind(
            &self
                .runtime_dir
                .get_socket()
                .expect("Implementar error de RootDir Path"),
        ) {
            Ok(listener) => Ok(listener),
            Err(err) => Err(ServiceError::new(
                ServiceErrorKind::FileCreationError,
                format!("Source: {:?}", err),
            )),
        }
    }

    /// Inicia el servicio.
    pub fn start(&mut self) -> Result<(), error::ServiceError> {
        for conn in self.make_socket()?.incoming() {
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

            let stop_iteration = match self.execute_command(&command) {
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
                break;
            }
        }
        Ok(())
    }

    fn execute_command(
        &mut self,
        command: &Command,
    ) -> Result<Response, error::ServiceError> {
        match command {
            INSTALL {
                target,
                kind,
                name,
                version,
            } => {
                println!("{:?}, {:?}, {:?}, {:?}", target, kind, name, version);
                Err(error::ServiceError::new(
                    error::ServiceErrorKind::NotImplementedError,
                    "Testeos, nada implementado por aquí!".to_string(),
                ))
            }
            DELETE { name, version } => {
                println!("{:?}, {:?}", name, version);
                Ok(Response::DELETE(Success))
            }
            SERVE { name, version } => {
                println!("{:?}, {:?}", name, version);
                Ok(Response::SERVE(Success))
            }
            HALT { name, version } => {
                println!("{:?}, {:?}", name, version);
                Ok(Response::HALT(Success))
            }
            STATUS => {
                println!("{:?}", command);
                Ok(Response::STATUS(vec![
                    Package::new(
                        "p1".to_string(),
                        "1.0.8".to_string(),
                        Generic,
                        Some(SocketAddrV4::new(
                            Ipv4Addr::new(127, 0, 0, 1),
                            8080,
                        )),
                    ),
                    Package::new(
                        "p2".to_string(),
                        "0.7.1".to_string(),
                        Generic,
                        None,
                    ),
                    Package::new(
                        "p3".to_string(),
                        "0.1.0".to_string(),
                        Generic,
                        Some("127.0.0.1:8080".parse().unwrap()),
                    ),
                    Package::new(
                        "p4".to_string(),
                        "3.2.0".to_string(),
                        Generic,
                        None,
                    ),
                    Package::new(
                        "p5".to_string(),
                        "8.0.9".to_string(),
                        Generic,
                        None,
                    ),
                ]))
            }
            EXIT => {
                println!("{:?}", command);
                Ok(Response::EXIT(Success))
            }
        }
    }
}

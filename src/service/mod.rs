// [MIT License] Copyright (c) 2024 Michel Novus

pub mod constants;
pub mod error;
pub mod process;
pub mod socket;

use process::RuntimeDir;
use socket::api::Command;
use socket::api::Command::*;
use socket::api::Response;
use socket::api::Status::*;

#[allow(dead_code)]
pub struct Service {
    runtime_dir: RuntimeDir,
}

impl Service {
    pub fn new(runtime_dir: RuntimeDir) -> Service {
        Service { runtime_dir }
    }

    pub fn execute_command(
        &mut self,
        command: Command,
    ) -> Result<Response, error::ServiceError> {
        match command {
            INSTALL {
                target,
                doc_type,
                doc_name,
                doc_version,
            } => {
                println!(
                    "{:?}, {:?}, {:?}, {:?}",
                    target, doc_type, doc_name, doc_version
                );
                Err(error::ServiceError::new(
                    error::ServiceErrorKind::Error1,
                    "Errooor".to_string(),
                ))
            }
            DELETE {
                doc_name,
                doc_version,
            } => {
                println!("{:?}, {:?}", doc_name, doc_version);
                Ok(Response::DELETE(Success))
            }
            SERVE {
                doc_name,
                doc_version,
            } => {
                println!("{:?}, {:?}", doc_name, doc_version);
                Ok(Response::SERVE(Success))
            }
            HALT {
                doc_name,
                doc_version,
            } => {
                println!("{:?}, {:?}", doc_name, doc_version);
                Ok(Response::HALT(Success))
            }
            STATUS => {
                println!("{:?}", command);
                Ok(Response::STATUS { status: Success })
            }
            EXIT => {
                println!("{:?}", command);
                Ok(Response::EXIT(Success))
            }
        }
    }
}

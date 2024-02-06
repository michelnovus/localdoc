// [MIT License] Copyright (c) 2024 Michel Novus

mod error;

use crate::socket::api::Command;
use crate::socket::api::Command::*;
use crate::socket::api::Response;
use crate::socket::api::Status::*;

pub struct Service {}

impl Service {
    pub fn new() -> Service {
        Service {}
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

// [MIT License] Copyright (c) 2024 Michel Novus

use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy)]
pub enum ServiceErrorKind {
    FileCreationError,
    NotImplementedError,
}

#[derive(Debug)]
pub struct ServiceError {
    kind: ServiceErrorKind,
    desc: String,
}

impl ServiceError {
    pub fn new(kind: ServiceErrorKind, desc: String) -> Self {
        ServiceError { kind, desc }
    }

    pub fn kind(&self) -> ServiceErrorKind {
        self.kind
    }
}

impl Error for ServiceError {}
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {:?}: {}", self.kind, &self.desc)
    }
}

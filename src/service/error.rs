// [MIT License] Copyright (c) 2024 Michel Novus

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ServiceErrorKind {
    Error1,
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

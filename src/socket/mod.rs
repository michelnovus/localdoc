// [MIT License] Copyright (c) 2024 Michel Novus

pub mod api;

use serde_json;
use std::io;
use std::io::prelude::*;
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

/// Lee el flujo de entrada del socket y lo traduce a estructuras Rust.
pub fn handle(stream: &mut UnixStream) -> io::Result<api::Command> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    stream.shutdown(Shutdown::Read)?;
    let command: api::Command = serde_json::from_str(&buffer)?;
    Result::Ok(command)
}

// [MIT License] Copyright (c) 2024 Michel Novus

use crate::api;
use serde_json;
use std::{io, io::prelude::*, net::Shutdown, os::unix::net::UnixStream};

/// Lee el flujo de entrada del socket y lo traduce a estructuras Rust.
pub fn recv(stream: &mut UnixStream) -> io::Result<api::Command> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    stream.shutdown(Shutdown::Read)?;
    let command: api::Command = serde_json::from_str(&buffer)?;
    Result::Ok(command)
}

/// Escribe en el flujo de salida del socket.
pub fn reply(
    stream: &mut UnixStream,
    message: api::Response,
) -> io::Result<()> {
    let mut buffer = String::new();
    buffer.push_str(&serde_json::to_string(&message)?);
    stream.write_all(buffer.as_bytes())?;
    stream.shutdown(Shutdown::Write)?;
    Ok(())
}

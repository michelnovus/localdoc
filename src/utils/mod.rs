// [MIT License] Copyright (c) 2024 Michel Novus

use crate::service::{process::RuntimeDir, socket::api::Command};
use serde_json;
use std::{io, io::prelude::*, net::Shutdown, os::unix::net::UnixStream};

/// Envia la señal `EXIT` al socket, para detener el servicio desde el exterior.
pub fn stop_service(runtime_dir: &RuntimeDir) -> io::Result<()> {
    let mut stream = UnixStream::connect(runtime_dir.get_socket().unwrap())?;
    {
        let command = serde_json::to_string(&Command::EXIT)?;
        stream.write_all(command.as_bytes())?;
        stream.shutdown(Shutdown::Write)?;
    }
    {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;
        stream.shutdown(Shutdown::Read)?;
        println!("{}", buffer);
    }
    Ok(())
}

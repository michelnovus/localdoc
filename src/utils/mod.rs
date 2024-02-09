// [MIT License] Copyright (c) 2024 Michel Novus

use crate::service::{
    constants::{ARG_START, ARG_STOP},
    process::RuntimeDir,
    socket::api::Command,
};
use serde_json;
use std::{
    io, io::prelude::*, net::Shutdown, os::unix::net::UnixStream, path::PathBuf,
};

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

/// Genera el contenido de un archivo unidad de systemd.
pub fn generate_unit_service(binary_path: PathBuf) -> String {
    let mut content = String::new();

    content.push_str("[Unit]\n");
    content.push_str(
        "Description=Inicia o detiene el servicio localdoc-service.\n",
    );

    content.push_str("\n[Service]\n");
    content.push_str("Type=forking\n");
    content.push_str(&format!(
        "ExecStart={} {}\n",
        binary_path.to_str().unwrap(),
        ARG_START
    ));
    content.push_str(&format!(
        "ExecStop={} {}\n",
        binary_path.to_str().unwrap(),
        ARG_STOP
    ));

    content.push_str("\n[Install]\n");
    content.push_str("WantedBy=multi-user.target\n");

    content
}

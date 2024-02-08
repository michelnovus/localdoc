// [MIT License] Copyright (c) 2024 Michel Novus
//! Se definen funciones y estructuras necesarias en el arranque de
//! un nuevo proceso.

use crate::service::{
    constants::{ROOT_DIR_NAME_DEFAULT, SOCKET_NAME_DEFAULT},
    socket::api::{Response, Response::EXIT, Status::Success},
};
use std::{fs, io, os::unix::fs::PermissionsExt, path::PathBuf};
use users::{get_current_uid, get_user_by_uid};

/// Resuelve la localización del directorio raíz del proceso en el sistema de archivos.
///
/// La función busca el `UID` del usuario activo y generar la ruta
/// `/run/user/$UID/localdoc` envolviendola con `Option::Some()`.
pub fn resolve_root_directory() -> Option<String> {
    match get_user_by_uid(get_current_uid()) {
        Some(data) => {
            let uid = data.uid();
            Some(String::from(format!(
                "/run/user/{uid}/{ROOT_DIR_NAME_DEFAULT}"
            )))
        }
        None => None,
    }
}

/// Comprueba si `response` es el valor `EXIT(Success)`.
pub fn stop_process(response: &Response) -> bool {
    if let EXIT(Success) = response {
        true
    } else {
        false
    }
}

/// Representa el directorio en tiempo de ejecución del proceso.
pub struct RuntimeDir {
    root_directory: PathBuf,
    socket_path: PathBuf,
    served_packages: PathBuf,
}

impl RuntimeDir {
    pub fn new(dirpath: &str) -> RuntimeDir {
        RuntimeDir {
            root_directory: PathBuf::from(dirpath),
            served_packages: PathBuf::from(&format!("{dirpath}/served")),
            socket_path: PathBuf::from(&format!(
                "{dirpath}/{SOCKET_NAME_DEFAULT}"
            )),
        }
    }

    /// Crea el directorio raiz de los datos del proceso en tiempo de ejecución.
    pub fn make(&self) -> io::Result<()> {
        if self.root_directory.is_absolute() && !self.root_directory.exists() {
            fs::create_dir_all(&self.root_directory)?;
            fs::metadata(&self.root_directory)?
                .permissions()
                .set_mode(0o700);
        }
        fs::create_dir(&self.served_packages)?;
        Ok(())
    }

    /// Devuelve la ruta absoluta al socket del proceso.
    pub fn get_socket(&self) -> Option<&PathBuf> {
        Some(&self.socket_path)
    }

    /// Devuelve la ruta absoluta al directorio raíz del proceso.
    pub fn get_root(&self) -> Option<&PathBuf> {
        Some(&self.root_directory)
    }
}

impl Drop for RuntimeDir {
    fn drop(&mut self) {
        fs::remove_file(&self.socket_path).unwrap_or_else(|err| {
            eprintln!("DROP ERROR [{:?}]: {:?}", err.kind(), &self.socket_path,)
        });
        fs::remove_dir(&self.served_packages).unwrap_or_else(|err| {
            eprintln!(
                "DROP ERROR [{:?}]: {:?}",
                err.kind(),
                &self.served_packages,
            )
        });
        fs::remove_dir(&self.root_directory).unwrap_or_else(|err| {
            eprintln!(
                "DROP ERROR [{:?}]: {:?}",
                err.kind(),
                &self.root_directory,
            )
        });
    }
}

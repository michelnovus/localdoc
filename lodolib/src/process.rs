// [MIT License] Copyright (c) 2024 Michel Novus
//! Se definen funciones y estructuras necesarias en el arranque de
//! un nuevo proceso.

use crate::api::{Response, Response::EXIT, Status::Success};
use crate::constants::{ROOT_DIR_NAME_DEFAULT, SOCKET_NAME_DEFAULT};
use std::{fs, io, os::unix::fs::PermissionsExt, path::PathBuf};
use users;

/// Busca el directorio /run/user/$UID en el sistema de archivos.
///
/// La función busca el `UID` del usuario activo y generar la ruta
/// `/run/user/$UID` envolviendola con `Option::Some()` si existe.
fn resolve_user_run_dir() -> Option<String> {
    match users::get_user_by_uid(users::get_current_uid()) {
        Some(data) => {
            let uid = data.uid();
            Some(String::from(format!("/run/user/{uid}")))
        }
        None => None,
    }
}
/// Busca el directorio /home/$USERNAME/.local/ en el sistema de archivos.
///
/// La función busca el `USERNAME` del usuario activo y generar la ruta
/// `/home/$USERNAME/.local/` envolviendola con `Option::Some()` si existe.
pub fn resolve_user_local_dir() -> Option<String> {
    match users::get_current_username() {
        Some(data) => {
            let username = data.to_string_lossy();
            Some(String::from(format!("/home/{username}/.local")))
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
    allow_drop: bool,
    root_directory: PathBuf,
    socket_path: PathBuf,
    served_doc_path: PathBuf,
}

impl RuntimeDir {
    pub fn new() -> io::Result<RuntimeDir> {
        let user_run_dir = match resolve_user_run_dir() {
            Some(value) => value,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "No se pudo resolver el directorio /run del usuario",
                ))
            }
        };
        let root_directory = format!("{user_run_dir}/{ROOT_DIR_NAME_DEFAULT}");

        Ok(RuntimeDir {
            root_directory: PathBuf::from(&root_directory),
            served_doc_path: PathBuf::from(&format!(
                "{root_directory}/served_doc"
            )),
            socket_path: PathBuf::from(&format!(
                "{root_directory}/{SOCKET_NAME_DEFAULT}"
            )),
            allow_drop: false,
        })
    }

    /// Comprueba si el directorio raíz existe.
    ///
    /// Si el directorio existe es por dos motivos:
    /// - Ya hay un servicio de Localdoc en ejecución
    /// - Existió en algún momento un servicio de Localdoc activo pero
    /// terminó de forma abrupta (mediante un SIGKILL por ejemplo) y no
    /// liberó recursos
    pub fn exists(&self) -> bool {
        self.root_directory.exists()
    }

    /// Crea el directorio raiz de los datos del proceso en tiempo de ejecución.
    pub fn make(&mut self) -> io::Result<()> {
        if self.root_directory.is_absolute() && !self.root_directory.exists() {
            fs::create_dir_all(&self.root_directory)?;
            fs::metadata(&self.root_directory)?
                .permissions()
                .set_mode(0o700);
        }
        fs::create_dir(&self.served_doc_path)?;
        self.allow_drop = true;
        Ok(())
    }

    /// Devuelve la ruta absoluta al socket del proceso.
    pub fn get_socket_path(&self) -> Option<&PathBuf> {
        Some(&self.socket_path)
    }

    /// Devuelve la ruta absoluta al directorio raíz del proceso.
    pub fn get_root_path(&self) -> Option<&PathBuf> {
        Some(&self.root_directory)
    }

    /// Devuelve la ruta absoluta al directorio de documentación servida.
    pub fn get_served_doc_path(&self) -> Option<&PathBuf> {
        Some(&self.root_directory)
    }
}

impl Drop for RuntimeDir {
    fn drop(&mut self) {
        if self.allow_drop {
            fs::remove_file(&self.socket_path).unwrap_or_else(|err| {
                eprintln!(
                    "DROP ERROR [{:?}]: {:?}",
                    err.kind(),
                    &self.socket_path,
                )
            });
            fs::remove_dir(&self.served_doc_path).unwrap_or_else(|err| {
                eprintln!(
                    "DROP ERROR [{:?}]: {:?}",
                    err.kind(),
                    &self.served_doc_path,
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
}

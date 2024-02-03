// [MIT License] Copyright (c) 2024 Michel Novus

//! Se definen funciones y estructuras necesarias en el arranque de
//! un nuevo proceso.

use std::env::var;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use users::{get_current_uid, get_user_by_uid};

/// Resuelve la localización del directorio raíz del proceso en el sistema de archivos.
///
/// La función busca una variable de entorno definida en `optional_envar` y
/// comprueba si no está vacía. Si tiene contenido la función retorna
/// un `Option::Some(String)` donde `String` es el contenido de la variable
/// de entrono.
///
/// Si `optional_envar` está vacía (no existe o es una cadena de
/// longitud cero), se intenta obtener el `UID` del usuario activo y
/// generar la ruta `/run/user/$UID/localdoc` envolviendola con `Option::Some()`.
///
/// Finalmente si no se pudieron obtener ninguna de las anteriores variables
/// de entrono la función devuelve `Option::None`.
pub fn resolve_root_directory(optional_envar: &str) -> Option<String> {
    let root_directory = var(optional_envar).unwrap_or_default();
    if root_directory.is_empty() {
        return match get_user_by_uid(get_current_uid()) {
            Some(data) => {
                let uid = data.uid();
                Some(String::from(format!("/run/user/{uid}/localdoc")))
            }
            None => None,
        };
    } else {
        Some(root_directory)
    }
}

/// Representa el directorio en tiempo de ejecución del proceso.
pub struct RuntimeDir {
    root_directory: PathBuf,
    socket_path: PathBuf,
}

impl RuntimeDir {
    pub fn new(dirpath: &str, socket_name: &str) -> RuntimeDir {
        RuntimeDir {
            root_directory: PathBuf::from(dirpath),
            socket_path: RuntimeDir::set_socket_path(dirpath, socket_name),
        }
    }

    /// Crea el directorio raiz de los datos del proceso en tiempo de ejecución.
    pub fn create_root(&self) -> io::Result<()> {
        if self.root_directory.is_absolute() && !self.root_directory.exists() {
            fs::create_dir_all(&self.root_directory)
        } else {
            io::Result::Err(Error::new(
                ErrorKind::AlreadyExists,
                format!("{:?} ya existe.", &self.root_directory),
            ))
        }
    }

    /// Devuelve la ruta absoluta al socket del proceso.
    pub fn get_socket(&self) -> &PathBuf {
        &self.socket_path
    }

    fn set_socket_path(root_path: &str, name: &str) -> PathBuf {
        let mut socket_path = PathBuf::new();
        socket_path.push(root_path);
        socket_path.push(name);
        socket_path
    }

    /// Devuelve la ruta absoluta al directorio raíz del proceso.
    pub fn get_root(&self) -> &PathBuf {
        &self.root_directory
    }
}

impl Drop for RuntimeDir {
    fn drop(&mut self) {
        fs::remove_file(&self.socket_path).unwrap_or_else(|err| {
            eprintln!("DROP ERROR [{:?}]: {:?}", err.kind(), &self.socket_path,)
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

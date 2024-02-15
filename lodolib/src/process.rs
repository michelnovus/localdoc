// [MIT License] Copyright (c) 2024 Michel Novus
//! Se definen funciones y estructuras necesarias en el arranque de
//! un nuevo proceso.

use crate::api::{Response, Response::EXIT, Status::Success};
use std::{fs, io, os::unix::fs::PermissionsExt, path::PathBuf};

/// Comprueba si `response` es el valor `EXIT(Success)`.
pub fn is_stop_command(response: &Response) -> bool {
    if let EXIT(Success) = response {
        true
    } else {
        false
    }
}

/// Representa los archivos y directorios del programa en el
/// sistema de ficheros del host.
pub struct AppFiles {
    /// Habilita la limpieza del directorio runtime del proceso.
    allow_drop: bool,

    /// Directorio de activos del proceso.
    runtime_dir_path: PathBuf,
    /// Directorio de documentación servida en curso.
    served_dir_path: PathBuf,
    /// Directorio de paquetes de documentación almacenada en disco.
    packages_path: PathBuf,

    /// Archivo socket del proceso.
    socket_path: PathBuf,
    /// Archivo de metadatos de los paquetes en `packages_path`.
    packages_meta_path: PathBuf,
    /// Archivo binario del servicio `lodosrv`.
    service_bin_path: Option<PathBuf>,
    /// Listado de posibles rutas al binario de `lodosrv`.
    service_bin_default_paths: Vec<PathBuf>,
}

impl AppFiles {
    pub fn new() -> io::Result<Self> {
        let runtime_dir = match dirs::runtime_dir() {
            Some(mut directory) => {
                directory.push("localdoc");
                directory
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "no se pudo obtener el directorio runtime del usuario",
                ))
            }
        };
        let persistent_dir =
            match dirs::data_local_dir() {
                Some(mut directory) => {
                    directory.push("localdoc");
                    directory
                }
                None => return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "no se pudo obtener el directorio local share del usuario",
                )),
            };
        Ok(AppFiles {
            allow_drop: false,
            runtime_dir_path: runtime_dir.clone(),
            served_dir_path: {
                let mut path = PathBuf::from(runtime_dir.clone());
                path.push("packages");
                path
            },
            packages_path: {
                let mut path = PathBuf::from(persistent_dir.clone());
                path.push("packages");
                path
            },
            socket_path: {
                let mut path = PathBuf::from(runtime_dir.clone());
                path.push("localdoc.socket");
                path
            },
            packages_meta_path: {
                let mut path = PathBuf::from(persistent_dir.clone());
                path.push("packages.json");
                path
            },
            service_bin_path: None,
            service_bin_default_paths: vec![],
        })
    }

    pub fn get_socket_path(&self) -> Option<&PathBuf> {
        if self.socket_path.exists() {
            Some(&self.socket_path)
        } else {
            None
        }
    }

    /// Asigna una lista de posibles rutas al binario de `lodosrv`.
    pub fn set_default_service_bin_paths(&mut self, defaults: Vec<PathBuf>) {
        self.service_bin_default_paths.clear();
        self.service_bin_default_paths.extend(defaults);
    }

    /// Devuelve la ruta absoluta del binario de `lodosrv` si existe.
    pub fn get_service_bin_path(&self) -> Option<&PathBuf> {
        if self.service_bin_path.is_some_and(|path| path.exists()) {
            self.service_bin_path.as_ref()
        } else {
            let mut iter =
                self.service_bin_default_paths.iter().filter_map(|path| {
                    if path.exists() {
                        Some(path.clone())
                    } else {
                        None
                    }
                });
            self.service_bin_path = iter.next();
            self.service_bin_path.as_ref()
        }
    }

    /// Crea el subdirectorio `localdoc` y su contenido en el
    /// runtime_dir del usuario.
    ///
    /// Si la función se ejecuta con éxito, permitirá al borrow checker
    /// eliminal todo el contenido generado en `runtime_dir/localdoc`.
    pub fn create_runtime(&mut self) -> io::Result<()> {
        if self.runtime_dir_path.exists() {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "El directorio `{}` ya existe",
                    &self.runtime_dir_path.to_string_lossy()
                ),
            ))
        } else {
            fs::create_dir_all(&self.runtime_dir_path)?;
            fs::metadata(&self.runtime_dir_path)?
                .permissions()
                .set_mode(0o700);
            fs::create_dir(&self.served_dir_path)?;
            self.allow_drop = true;
            Ok(())
        }
    }
}

impl std::ops::Drop for AppFiles {
    fn drop(&mut self) {
        if self.allow_drop {
            fs::remove_file(&self.socket_path).unwrap_or_else(|err| {
                eprintln!(
                    "DROP ERROR [{:?}]: {:?}",
                    err.kind(),
                    &self.socket_path,
                )
            });
            fs::remove_dir_all(&self.served_dir_path).unwrap_or_else(|err| {
                eprintln!(
                    "DROP ERROR [{:?}]: {:?}",
                    err.kind(),
                    &self.served_dir_path,
                )
            });
            fs::remove_dir(&self.runtime_dir_path).unwrap_or_else(|err| {
                eprintln!(
                    "DROP ERROR [{:?}]: {:?}",
                    err.kind(),
                    &self.runtime_dir_path,
                )
            });
        };
    }
}

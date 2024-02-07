// [MIT License] Copyright (c) 2024 Michel Novus
//! Comandos, respuestas y serializacion del Socket API del proceso.

use serde::{Deserialize, Serialize};
use std::{net::SocketAddrV4, path::PathBuf};

/// Paquete de información sobre documentación.
#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    version: String,
    kind: DocKind,
    /// Some(http://addr:port) o None
    url: Option<SocketAddrV4>,
}

impl Package {
    pub fn new(
        name: String,
        version: String,
        kind: DocKind,
        url: Option<SocketAddrV4>,
    ) -> Self {
        Package {
            name,
            version,
            kind,
            url,
        }
    }
}

/// Estado de ejecución binario, éxito o fracaso.
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Failed,
    Success,
}

/// Define los tipos de documentación.
#[derive(Serialize, Deserialize, Debug)]
pub enum DocKind {
    /// Servir la documentación como un sitio web estático común.
    Generic,
    /// Servir la documentación mediante el motor MdBook.
    MdBook,
}

/// Enumeracion que define los comandos disponibles de la API.
///
/// Los comandos son provistos por el cliente, el cual ordena que debe
/// intentar hacer el servicio.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Ordena instalar un nuevo paquete de documentación.
    INSTALL {
        /// Ruta absoluta a la raíz del directorio de la documentación.
        target: PathBuf,
        /// Es el tipo de docuentación, ver enum `DocKind`
        kind: DocKind,
        /// El nombre que se quiere para identificar la documentación.
        name: String,
        /// El número de versión de la documentación.
        version: String,
    },
    /// Ordena desinstalar un paquete de documentación.
    DELETE {
        /// Nombre identificativo del paquete.
        name: String,
        /// Versión del paquete.
        version: String,
    },
    /// Ordena servir un paquete de documentación.
    SERVE {
        /// Nombre identificativo del paquete.
        name: String,
        /// Versión del paquete.
        version: String,
    },
    /// Ordena dejar de servir un paquete de documentación.
    HALT {
        /// Nombre identificativo del paquete.
        name: String,
        /// Versión del paquete.
        version: String,
    },
    /// Solicita toda la información de estado actual del servicio.
    STATUS,
    /// Orena terminar el servicio.
    EXIT,
}

/// Define las respuestas que da el servicio al cliente.
///
/// Las variantes estan relacionadas 1 a 1 con la `enum Command`.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    INSTALL(Status),
    DELETE(Status),
    SERVE(Status),
    HALT(Status),
    STATUS(Vec<Package>),
    EXIT(Status),
}

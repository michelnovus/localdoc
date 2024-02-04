// [MIT License] Copyright (c) 2024 Michel Novus
//! Comandos, errores, serializacion del Socket API del proceso.

use serde::{Deserialize, Serialize};

/// Estado de ejecución binario, éxito o fracaso.
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Failed,
    Success,
}

/// Enumeracion que define los comandos disponibles de la API.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    //TODO: Implementar los atributos necesarios.
    INSTALL {},
    DELETE {},
    SERVE {},
    HALT {},
    STATUS,
    EXIT,
}

/// Enumeracion que define las respuestad a los comandos de la API.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    //TODO: Implementar los atributos necesarios.
    INSTALL(Status),
    DELETE(Status),
    SERVE(Status),
    HALT(Status),
    STATUS { status: Status },
    EXIT(Status),
}

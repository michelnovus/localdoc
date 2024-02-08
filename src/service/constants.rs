// [MIT License] Copyright (c) 2024 Michel Novus
//! Define constantes glbales del programa.

/// Nombre de la variable de entorno que define el directorio raíz del proceso.
pub const ROOT_DIRECTORY_ENV_VAR: &'static str = "LOCALDOC_ROOT";
/// Nombre de la variable de entorno que define el nombre del archivo socket.
pub const SOCKET_NAME_ENV_VAR: &'static str = "LOCALDOC_SOCKET";

/// Nombre predeterminado del socket.
pub const SOCKET_NAME_DEFAULT: &'static str = "localdoc.socket";
/// Nombre predeterminado de la raíz.
pub const ROOT_DIR_NAME_DEFAULT: &'static str = "localdoc";

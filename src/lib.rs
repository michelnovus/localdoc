// [MIT License] Copyright (c) 2024 Michel Novus

use std::env::var;
use users::{get_current_uid, get_user_by_uid};

/// Resuelve la localización del socket del proceso en el sistema de archivos.
///
/// La función busca una variable de entorno llamada `LOCALDOC_SOCKET` y
/// comprueba si no está vacía. Si tiene contenido la función retorna
/// un `Option::Some(String)` donde `String` es el contenido de la variable
/// de entrono.
///
/// Si `LOCALDOC_SOCKET` está vacía (no existe o es una cadena de
/// longitud cero), se intenta obtener el `UID` del usuario activo y
/// generar la ruta `/run/user/$UID/localdoc/localdoc.socket` envolviendola
/// en un `Option::Some()`.
///
/// Finalmente si no se pudieron obtener ninguna de las anteriores variables
/// de entrono la función devuelve `Option::None`.
pub fn resolve_socket_filepath(optional_envar: &str) -> Option<String> {
    let socket_filepath = var(optional_envar).unwrap_or_default();
    if socket_filepath.is_empty() {
        return match get_user_by_uid(get_current_uid()) {
            Some(data) => {
                let uid = data.uid();
                Some(String::from(format!(
                    "/run/user/{uid}/localdoc/localdoc.socket"
                )))
            }
            None => None,
        };
    } else {
        Some(socket_filepath)
    }
}

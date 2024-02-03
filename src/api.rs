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

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_commands() {
        let deserialized: Command =
            serde_json::from_str(r#"{"INSTALL":{}}"#).unwrap();
        assert!(matches!(deserialized, Command::INSTALL {}));

        let deserialized: Command =
            serde_json::from_str(r#"{"DELETE":{}}"#).unwrap();
        assert!(matches!(deserialized, Command::DELETE {}));

        let deserialized: Command =
            serde_json::from_str(r#"{"SERVE":{}}"#).unwrap();
        assert!(matches!(deserialized, Command::SERVE {}));

        let deserialized: Command =
            serde_json::from_str(r#"{"HALT":{}}"#).unwrap();
        assert!(matches!(deserialized, Command::HALT {}));

        let deserialized: Command =
            serde_json::from_str(r#""STATUS""#).unwrap();
        assert!(matches!(deserialized, Command::STATUS));

        let deserialized: Command = serde_json::from_str(r#""EXIT""#).unwrap();
        assert!(matches!(deserialized, Command::EXIT));
    }

    #[test]
    #[should_panic]
    fn test_unknown_command() {
        let _des: Command = serde_json::from_str(r#"{"NO_COMMAND"}"#).unwrap();
        let _des: Command = serde_json::from_str("").unwrap();
        let _des: Command = serde_json::from_str("22").unwrap();
        let _des: Command = serde_json::from_str(r#"{"INSTALL": 22}"#).unwrap();
    }

    #[test]
    fn test_responses() {
        let res = Response::EXIT(Status::Failed);
        let res = serde_json::to_string(&res).unwrap();
        assert_eq!(res, r#"{"EXIT":"Failed"}"#);

        let res = Response::STATUS {
            status: Status::Success,
        };
        let res = serde_json::to_string(&res).unwrap();
        assert_eq!(res, r#"{"STATUS":{"status":"Success"}}"#);
    }
}

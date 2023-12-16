# [GLP3] Copyright (C) 2024  Michel Novus
"""Define la interfáz del cliente."""

import sys

from configuration import Config
from .console import console
from .arguments import OPT, CMD, arguments, print_help
from .execution_tasks import List, Serve, Close, Install, Remove, CloseDaemon


def run_client(configuration: Config) -> None:
    """Ejecuta el proceso interactivo."""
    args = arguments()
    if len(args) == 0:
        print_help()
        sys.exit(1)

    match args["CMD"]:
        case CMD.LIST:
            List(configuration, [], [])
        case CMD.SERVE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            if OPT.OPEN in args["OPT"]:
                open_in_browser = [OPT.OPEN]
            else:
                open_in_browser = []
            Serve(configuration, args["ARG"], open_in_browser)
        case CMD.CLOSE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            Close(configuration, args["ARG"], [])
        case CMD.INSTALL:
            if len(args["ARG"]) == 0:
                console.print(
                    "Debe pasar como argumento por lo menos un directorio "
                    "que contenga un sitio web."
                )
                sys.exit(1)
            Install(configuration, args["ARG"], [])
        case CMD.REMOVE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            Remove(configuration, args["ARG"], [])
        case CMD.CLOSE_DAEMON:
            CloseDaemon(configuration, [], [])
        case _:
            raise NotImplementedError(
                f"El comando {args['CMD']} es inválido o no está implementado."
            )

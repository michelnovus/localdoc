# [GLP3] Copyright (C) 2024  Michel Novus
"""Define la interfáz del cliente."""

import sys

from configuration import Config
from .console import console
from .arguments import OPT, CMD, arguments, print_help
from .execution_tasks import *


def run_client(cfg: Config) -> None:
    """Ejecuta el proceso interactivo."""
    args = arguments()
    if len(args) == 0:
        print_help()
        sys.exit(1)

    match args["CMD"]:
        case CMD.LIST:
            show_listing_wall(cfg.socket_filepath, cfg.package_dir)
        case CMD.SERVE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            if OPT.OPEN in args["OPT"]:
                open_in_browser = True
            else:
                open_in_browser = False
            for package in args["ARG"].values():
                serve_package(cfg.socket_filepath, package)
        case CMD.CLOSE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            for package in args["ARG"]:
                close_package(cfg.socket_filepath, package)
        case CMD.INSTALL:
            if len(args["ARG"]) == 0:
                console.print(
                    "Debe pasar como argumento por lo menos un directorio "
                    "que contenga un sitio web."
                )
                sys.exit(1)
            for package in args["ARG"]:
                install_package(package)
        case CMD.REMOVE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            for package in args["ARG"]:
                uninstall_package(package)
        case CMD.CLOSE_DAEMON:
            close_daemon(cfg.socket_filepath)
        case _:
            raise NotImplementedError(
                f"El comando {args['CMD']} es inválido o no está implementado."
            )

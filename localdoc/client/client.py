# [GLP3] Copyright (C) 2024  Michel Novus
"""Define la interfáz del cliente."""

import sys

from configuration import Config
from .console import console
from .arguments import OPT, CMD, arguments, print_help
from packages import get_available_packages, get_served_packages
from .execution_tasks import *


def run_client(cfg: Config) -> None:
    """Ejecuta el proceso interactivo."""
    args = arguments()
    if len(args) == 0:
        print_help()
        sys.exit(1)

    # available_packages = get_available_packages(cfg.package_dir)
    # served_packages = get_served_packages(cfg.socket_filepath)
    available_packages = get_available_packages(cfg.package_dir)
    served_packages = {}
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
                if package in available_packages:
                    serve_package(cfg.socket_filepath, package)
                else:
                    console.print(
                        f"El paquete [b]{package}[/b] no está disponible."
                    )
        case CMD.CLOSE:
            if len(args["ARG"]) == 0:
                console.print(
                    "Faltan argumentos! Vea el comando [b]list[/b] para "
                    "obtener los argumentos válidos."
                )
                sys.exit(1)
            for package in args["ARG"]:
                if package in served_packages:
                    close_package(cfg.socket_filepath, package)
                else:
                    console.print(
                        f"El paquete [b]{package}[/b] no esta actualmente servido."
                    )
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
                if package in available_packages:
                    uninstall_package(package)
                else:
                    console.print(
                        f"Nombre de paquete [b]{package}[/b] inválido."
                    )
        case CMD.CLOSE_DAEMON:
            close_daemon(cfg.socket_filepath)
        case _:
            raise NotImplementedError(
                f"El comando {args['CMD']} es inválido o no está implementado."
            )

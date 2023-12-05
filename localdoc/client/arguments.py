# [GLP3] Copyright (C) 2024  Michel Novus
"""Analiza los argumentos que se pasan al cliente en la Línea de Comandos."""

from typing import Any
from enum import Enum
import sys

from .console import console


CMD = Enum(
    "CMD", ["LIST", "SERVE", "CLOSE", "INSTALL", "REMOVE", "CLOSE_DAEMON"]
)
OPT = Enum("OPT", ["OPEN"])


def arguments() -> dict[str, Any]:
    """Devuelve un dict que contiene la tarea y los datos que debe
    ejecutar el proceso.

    Formato de salida:
    - siempre {"CMD": CMD.TYPE}
    - opcional {..., "ARG": [], "OPT": []}

    Cada comando tiene por salida:
    - list: {"CMD": CMD.LIST}
    - serve: {"CMD": CMD.SERVE, "OPT": [], "ARG": []}
    - close: {"CMD": CMD.CLOSE, "ARG": []}
    - install: {"CMD": CMD.INSTALL, "ARG": []}
    - remove: {"CMD": CMD.REMOVE, "ARG": []}
    - close-daemon: {"CMD": CMD.CLOSE_DAEMON}

    El dict está vacío si no existe argumento válido.
    """
    args = sys.argv
    if len(args) == 1:
        _print_help()
        return {}
    else:
        args.extend("" * 5)

    match args[1]:
        case "list":
            return {"CMD": CMD.LIST}
        case "serve":
            serve_command = {"CMD": CMD.SERVE, "OPT": [], "ARG": []}
            if "--open" in args:
                args.remove("--open")
                serve_command["OPT"].append(OPT.OPEN)
            for arg in args[2:]:
                serve_command["ARG"].append(arg)
            return serve_command
        case "close":
            close_command = {"CMD": CMD.CLOSE, "ARG": []}
            for arg in args[2:]:
                close_command["ARG"].append(arg)
            return close_command
        case "install":
            install_command = {"CMD": CMD.INSTALL, "ARG": []}
            for arg in args[2:]:
                install_command["ARG"].append(arg)
            return install_command
        case "remove":
            remove_command = {"CMD": CMD.REMOVE, "ARG": []}
            for arg in args[2:]:
                remove_command["ARG"].append(arg)
            return remove_command
        case "close-daemon":
            return {"CMD": CMD.CLOSE_DAEMON}
        case "-h" | "--help" | "help":
            _print_help()
            return {}
        case _:
            _invalid_command_notify(args[1], "cmd")
            console.print()
            _print_help()
            return {}


def _print_help() -> None:
    """Imprime la pantalla de ayuda."""
    text = [
        "[u][b]Uso:[/u] localdoc[/b] COMANDO [ argumentos ] [ --opciones ]",
        "",
        "[u b]Comandos:[/]",
        "  [b]list[/]             Muestra una lista de toda la documentación",
        "                   disponible en la base de datos.",
        "  [b]serve[/]            Inicia el servicio de documentación en",
        "                   un puerto, utilizando el argumento.",
        "                   proporcionado si esta disponible (ver [b]list[/]).",
        "  [b]close[/]            Cierra el o los servicios que se pasen",
        "                   como argumento.",
        "  [b]install[/]          Instala la documentación en la base de datos.",
        "  [b]remove[/]           Remueve la documentación de la base de datos.",
        "  [b]close-daemon[/]     Mata el proceso daemon.",
        "  [b]help[/]             Muestra esta ayuda.",
        "",
        "[u b]Opciones:[/]",
        "  [b]--open[/]           Abre el servicio en el navegador, acompaña",
        "                   al comando [b]serve[/].",
        "  [b]-h, --help[/]       Alias de [b]help[/].",
        "",
    ]
    for string in text:
        console.print(string)


def _invalid_command_notify(arg: str, type: str) -> None:
    """Imprime un aviso de comando, argumento o bandera inválida.

    El parámetro type puede ser: 'cmd', 'arg' o 'flg'
    """
    match type:
        case "cmd":
            console.print(
                f"ARGUMENTO INVÁLIDO: [red]{arg}[/red], "
                f"se espearaba un [green]COMANDO[/green]."
            )
        case "arg":
            console.print(
                f"ARGUMENTO INVÁLIDO: [red]{arg}[/red], "
                f"se espearaba un [green]argumento[/green]."
            )
        case "flg":
            console.print(
                f"ARGUMENTO INVÁLIDO: [red]{arg}[/red], "
                f"se espearaba una [green]--opción[/green]."
            )
        case _:
            raise ValueError(f"{type} no es válido!")

# [GLP3] Copyright (C) 2024  Michel Novus
"""Analiza los argumentos que se pasan al cliente en la Línea de Comandos."""

from typing import Any
import sys

from .console import console


def arguments() -> dict[str, Any]:
    """Devuelve un dict que contiene la tarea y los datos que debe
    ejecutar el proceso.

    El dict está vacío si no existe argumento válido.
    """
    args = sys.argv
    if len(args) == 1:
        _print_help()
        return {}
    else:
        for i in range(5):
            args.append("")

    match args[1]:
        case "list":
            return {"CMD": "list"}
        case "serve":
            return {"CMD": "serve"}
        case "close":
            return {"CMD": "close"}
        case "install":
            return {"CMD": "install"}
        case "remove":
            return {"CMD": "remove"}
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
        "  [b]list[/]          Lista la documentación disponible",
        "  [b]serve[/]         Carga la documentación nombrada",
        "  [b]close[/]         Cierra la documentación nombrada",
        "  [b]install[/]       Instala la documentación en la base de datos",
        "  [b]remove[/]        Remueve la documentación de la base de datos",
        "  [b]help[/]          Muestra esta ayuda",
        "",
        "[u b]Opciones:[/]",
        "  [b]--open[/]        Abre la documentación en el navegador",
        "  [b]--close-daemon[/]Mata al proceso daemon",
        "  [b]-h, --help[/]    Alias de help",
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

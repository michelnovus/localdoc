# [GLP3] Copyright (C) 2024  Michel Novus
"""Instancia de Console del módulo Rich."""

from rich.console import Console


console = Console()
"""Representa la TUI."""


class Show(object):
    """Funciones que devuelven un aviso de estado en la forma: [ ESTADO ]"""

    @staticmethod
    def ok(message: str) -> str:
        return f"[  [green]OK[/]  ] {message}"

    @staticmethod
    def fail(message: str) -> str:
        return f"[ [red]FAIL[/] ] {message}"

    @staticmethod
    def warn(message: str) -> str:
        return f"[ [yellow]WARN[/] ] {message}"

    @staticmethod
    def info(message: str) -> str:
        return f"[ [green]INFO[/] ] {message}"

    @staticmethod
    def space(message: str) -> str:
        return f"         {message}"

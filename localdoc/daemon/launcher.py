# [GLP3] Copyright (C) 2024  Michel Novus
"""Daemonizador de localdocd."""

import subprocess
import os.path
import sys

import rich


def daemonize() -> None:
    """Ejecuta el archivo 'localdocd' como un proceso que se mantendrá
    vivo en segundo plano.

    Lanza PermissionError si localdocd no tiene permiso de ejecución.
    """
    launcher_path = os.path.abspath(__file__)
    localdocd_file = os.path.join(os.path.split(launcher_path)[0], "localdocd")
    subprocess.Popen(
        [localdocd_file],
        shell=False,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        close_fds=True,
        cwd="/",
    )


if __name__ == "__main__":
    try:
        daemonize()
    except PermissionError:
        rich.print(
            "[ [red bold]FAIL[/red bold] ] Error al iniciar: "
            "localdocd, ¡no tiene permiso de ejecución!"
        )
        sys.exit(1)

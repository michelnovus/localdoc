# [GLP3] Copyright (C) 2024  Michel Novus

import sys
import os
import os.path

from client import console, run_client
from client import config


CONFIG_DIRPATH = os.path.join(os.getenv("HOME", os.getcwd()), ".local/localdoc")
CONFIG_FILEPATH = os.path.join(CONFIG_DIRPATH, "localdoc.toml")
SOCKET, PORT = ("127.0.0.2", 21980)
PACKAGE_DIRECTORY = os.path.join(CONFIG_DIRPATH, "packages")


def main() -> None:
    _make_config_dirs()
    _make_config_file_if_not_exist()
    configuration: config.Config = _load_configuration()


def _make_config_dirs() -> None:
    """Crea los directorios de configuración y de paquetes."""
    if not os.path.exists(CONFIG_DIRPATH):
        try:
            os.makedirs(CONFIG_DIRPATH, exist_ok=True)
            console.print(
                "[  [green]OK[/green]  ] Se creó el directorio de "
                "configuración de programa."
            )
        except PermissionError:
            console.print(
                "[ [red]FAIL[/red] ] No tiene permisos en el sistema "
                "para crear el directorio de configuración!"
            )
    if not os.path.exists(PACKAGE_DIRECTORY):
        try:
            os.makedirs(PACKAGE_DIRECTORY, exist_ok=True)
            console.print(
                "[  [green]OK[/green]  ] Se creó el directorio de "
                "de paquetes del programa."
            )
        except PermissionError:
            console.print(
                "[ [red]FAIL[/red] ] No tiene permisos en el sistema "
                "para crear el directorio de paquetes!"
            )


def _make_config_file_if_not_exist() -> None:
    """Crea el archivo de configuración si el archivo no existe.

    Cierra el proceso ante fallos.
    """
    try:
        with open(CONFIG_FILEPATH, "xt") as cfg_file:
            cfg_file.write(config.new(SOCKET, PORT, PACKAGE_DIRECTORY))
    except FileExistsError:
        pass
    except OSError:
        console.print(
            "[ [red]FAIL[/red] ] Error al crear el archivo de configuración"
        )
        sys.exit(1)


def _load_configuration() -> config.Config:
    """Carga la configuración desde CONFIG_FILEPATH
    y devuelve el objeto Config.

    Cierra el proceso ante fallos.
    """
    try:
        with open(CONFIG_FILEPATH, "rt") as cfg_file:
            configuration = config.Config(cfg_file.read())
    except OSError:
        console.print(
            "[ [red]FAIL[/red] ] Error al abrir el archivo de configuración"
        )
        sys.exit(1)
    except config.TOMLDecodeError:
        console.print(
            "[ [red]FAIL[/red] ] El archivo de configuración no es válido!"
        )
        sys.exit(1)
    return configuration


if __name__ == "__main__":
    main()

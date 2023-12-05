# [GLP3] Copyright (C) 2024  Michel Novus

import sys
import os
import os.path
from tempfile import mkdtemp

from default import CONFIG_DIRPATH, CONFIG_FILEPATH, PACKAGE_DIRECTORY
import config
from daemon import daemonize, clear_tempdir, localdocd_is_running
from client import console, run_client


def main() -> None:
    # -----------------------------------------------------------------
    # Preparación del entorno de ejecución del programa.
    # Inicializa la configuración si no existe creando los directorios
    # predeterminados en el directorio HOME del usuario.
    # Inicia el proceso localdocd si no está ejecutándose.

    _make_config_directory()
    _make_config_file()
    conf: config.Config = _load_configuration()
    if not localdocd_is_running(conf.socket_filepath):
        clear_tempdir(os.path.split(conf.socket_filepath)[0])
        socket_dirpath = mkdtemp(prefix="localdoc-", dir="/tmp")
        os.chmod(socket_dirpath, 0o700)
        with open(CONFIG_FILEPATH, "wt") as config_file:
            config_file.write(
                config.new(
                    socket=f"{socket_dirpath}/localdoc.socket",
                    package_dir=PACKAGE_DIRECTORY,
                )
            )
        conf: config.Config = _load_configuration()
        daemonize()
    _make_package_directory(conf.package_dir)
    # -----------------------------------------------------------------

    run_client(configuration=conf)


def _make_config_directory() -> None:
    """Crea el directorio de configuración si no existe."""
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


def _make_config_file() -> None:
    """Crea el archivo de configuración si el archivo no existe.

    Cierra el proceso ante fallos.
    """
    try:
        with open(CONFIG_FILEPATH, "xt") as cfg_file:
            cfg_file.write(
                config.new(socket="undefined", package_dir=PACKAGE_DIRECTORY)
            )
    except FileExistsError:
        pass
    except OSError:
        console.print(
            "[ [red]FAIL[/red] ] Error al crear el archivo de configuración"
        )
        sys.exit(1)


def _make_package_directory(path: str) -> None:
    """Crea el directorio de los paquetes si no existe."""
    if not os.path.exists(path):
        try:
            os.makedirs(path, exist_ok=True)
            console.print(
                "[  [green]OK[/green]  ] Se creó el directorio de "
                "de paquetes del programa."
            )
        except PermissionError:
            console.print(
                "[ [red]FAIL[/red] ] No tiene permisos en el sistema "
                "para crear el directorio de paquetes!"
            )


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

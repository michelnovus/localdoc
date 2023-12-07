# [GLP3] Copyright (C) 2024  Michel Novus

import sys
import os
import os.path
import time
from tempfile import mkdtemp

from default import CONFIG_DIRPATH, CONFIG_FILEPATH, PACKAGE_DIRECTORY
import config
from daemon import daemonize, clear_tempdir, localdocd_is_running
from client import console, Show, run_client


def main() -> None:
    # -----------------------------------------------------------------
    # Preparación del entorno de ejecución del programa.
    # Inicializa la configuración si no existe creando los directorios
    # predeterminados en el directorio HOME del usuario.
    # Inicia el proceso localdocd si no está ejecutándose.

    _make_config_directory()
    _make_config_file()
    conf: config.Config = _load_configuration()
    if not localdocd_is_running(conf.socket_filepath) or not os.path.exists(
        conf.socket_filepath
    ):
        console.print(Show.warn("No esta cargado el proceso [b]localdocd[/]."))
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
        console.print(Show.space("Intentando iniciar [b]localdocd[/]."))
        daemonize()
        time.sleep(1)
        if localdocd_is_running(conf.socket_filepath):
            console.print(Show.ok("[b]localdocd[/] se inició con éxito!"))
            console.print()
            time.sleep(0.5)
        else:
            console.print(Show.fail("[b]localdocd[/] no se pudo iniciar."))
            sys.exit(1)
    _make_package_directory(conf.package_dir)
    # -----------------------------------------------------------------
    run_client(configuration=conf)


def _make_config_directory() -> None:
    """Crea el directorio de configuración si no existe."""
    if not os.path.exists(CONFIG_DIRPATH):
        try:
            os.makedirs(CONFIG_DIRPATH, exist_ok=True)
            console.print(
                Show.ok(
                    "Se creó el directorio de " "configuración de programa."
                )
            )
        except PermissionError:
            console.print(
                Show.fail(
                    "No tiene permisos en el sistema "
                    "para crear el directorio de configuración!"
                )
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
        console.print(Show.ok("Creado nuevo archivo de configuración."))
    except FileExistsError:
        pass
    except OSError:
        console.print(Show.fail("Error al crear el archivo de configuración."))
        sys.exit(1)


def _make_package_directory(path: str) -> None:
    """Crea el directorio de los paquetes si no existe."""
    if not os.path.exists(path):
        try:
            os.makedirs(path, exist_ok=True)
            console.print(
                Show.ok(
                    " Se creó el directorio de " "de paquetes del programa."
                )
            )
        except PermissionError:
            console.print(
                Show.fail(
                    "No tiene permisos en el sistema "
                    "para crear el directorio de paquetes!"
                )
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
        console.print(Show.fail("Error al abrir el archivo de configuración."))
        sys.exit(1)
    except config.TOMLDecodeError:
        console.print(Show.fail("El archivo de configuración no es válido!"))
        sys.exit(1)
    return configuration


if __name__ == "__main__":
    main()

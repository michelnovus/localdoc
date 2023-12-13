# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path
import sys
import time
import tempfile
from tomllib import TOMLDecodeError

from client import console, Show
from daemon import clear_tempdir, daemonize, is_localdocd_running
from default import CONFIG_DIRPATH, CONFIG_FILEPATH, PACKAGE_DIRECTORY
from configuration import generate_new_configuration, load_configuration, Config


def mk_main_configuration_directory() -> None:
    """Genera si no existe el directorio de configuración principal de
    localdoc.

    Si no se puede crear el directorio aborta el proceso.
    """
    if not os.path.exists(CONFIG_DIRPATH):
        try:
            os.makedirs(CONFIG_DIRPATH, exist_ok=True)
            console.print(
                Show.ok("Se creó el directorio de configuración de programa.")
            )
        except PermissionError:
            console.print(
                Show.fail(
                    "No tiene permisos en el sistema "
                    "para crear el directorio de configuración!"
                )
            )
            sys.exit(1)


def mk_main_configuration_file() -> None:
    """Genera si no existe el archivo de configuración principal de
    localdoc.

    Si no se puede crear el archivo aborta el proceso.
    """
    try:
        with open(CONFIG_FILEPATH, "xt") as cfg_file:
            cfg_file.write(
                generate_new_configuration(
                    socket_filepath="undefined", package_dir=PACKAGE_DIRECTORY
                )
            )
        console.print(Show.ok("Creado nuevo archivo de configuración."))
    except FileExistsError:
        pass
    except OSError:
        console.print(Show.fail("Error al crear el archivo de configuración."))
        sys.exit(1)


def mk_package_directory(path: str) -> None:
    """Crea el directorio de los paquetes, si no existe no hace nada.

    Si no puede crear el directorio aborta el proceso.
    """
    if not os.path.exists(path):
        try:
            os.makedirs(path, exist_ok=True)
            console.print(
                Show.ok(
                    " Se creó el directorio de de paquetes del programa "
                    f"en {path}"
                )
            )
        except PermissionError:
            console.print(
                Show.fail(
                    "No tiene permisos en el sistema "
                    "para crear el directorio de paquetes!"
                )
            )
            sys.exit(1)


def fetch_data_from_configuration_file() -> str:
    """Abre y carga en memoria el contenido del archivo de configuración
    principal de localdoc.

    Genera las excepciones de la función built-in 'open'.
    """
    cfg_file = open(CONFIG_FILEPATH, "rt")
    data = cfg_file.read()
    cfg_file.close()
    return data


def load_configuration_with_notices(exit_on_error: bool = True) -> Config:
    """Envoltorio de 'load_configuration' manejando las excepciones e
    imprimiendo avisos en la terminal."""
    if not exit_on_error:
        return load_configuration(fetch_data_from_configuration_file())
    try:
        configuration = load_configuration(fetch_data_from_configuration_file())
    except TOMLDecodeError:
        console.print(Show.fail("El archivo de configuración no es válido!"))
        sys.exit(1)
    except OSError:
        console.print(Show.fail("Error al abrir el archivo de configuración."))
        sys.exit(1)
    except KeyError:
        console.print(Show.fail("Faltan claves en el archivo!"))
        sys.exit(1)
    else:
        return configuration


def init_daemon(ipc_socket: str) -> None:
    """Prepara e inicia un proceso independiente de localdocd."""

    console.print(Show.space("limpiando socket antiguo..."))
    clear_tempdir(os.path.split(ipc_socket)[0])
    console.print(Show.space("generando nuevo socket..."))
    socket_dirpath = tempfile.mkdtemp(prefix="localdoc-", dir="/tmp")
    os.chmod(socket_dirpath, 0o700)
    console.print(Show.space("generando nuevo archivo de configuración..."))
    with open(CONFIG_FILEPATH, "wt") as config_file:
        config_file.write(
            generate_new_configuration(
                socket_filepath=f"{socket_dirpath}/localdoc.socket",
                package_dir=PACKAGE_DIRECTORY,
            )
        )
    configuration = load_configuration_with_notices()
    console.print(Show.space("Intentando iniciar [b]localdocd[/]."))
    daemonize()
    time.sleep(1)
    if is_localdocd_running(configuration.socket_filepath):
        console.print(Show.ok("[b]localdocd[/] se inició con éxito!"))
        time.sleep(0.5)
        console.print()
    else:
        console.print(Show.fail("[b]localdocd[/] no se pudo iniciar."))
        sys.exit(1)

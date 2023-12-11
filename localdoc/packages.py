# [GLP3] Copyright (C) 2024  Michel Novus
"""Información de paquetes."""

import os
from typing import Sequence
import filetype


def available_packages(
    package_directory: str, types: Sequence[str] = ("tar", "xz", "bz2")
) -> list[str]:
    """Devuelve una lista de las rutas absolutas a los paquetes disponibles.

    ### Caracteristicas de los paquetes:
    - se espera que los nombres de los paquetes no tengan espacios
    - solo archivos de tipo especificados en el parametro 'types'
    - solo se listan los archivos en 'package_directory'
    - y solo se listan los archivos accesibles
    """
    packages = []
    with os.scandir(package_directory) as package_dir:
        for entry in package_dir:
            if entry.is_file():
                if " " in entry.name:
                    continue
                try:
                    access = open(entry.path, "rb")
                    access.close()
                    kind = filetype.guess_extension(entry.path)
                except (TypeError, OSError, PermissionError):
                    continue
                if kind in types:
                    packages.append(entry.path)
    packages.sort()
    return packages


def served_packages(ipc_socket) -> dict[str, tuple[str, int]]:
    """Comprueba los paquetes que están servidos actualmente en algún puerto.
    Retorna un 'dict' que tiene por claves los nombres de los paquetes y por
    valores una tupla con el socket web y el puerto.
    """
    return {}

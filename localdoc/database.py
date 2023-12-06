# [GLP3] Copyright (C) 2024  Michel Novus

import os
from typing import Sequence

from config import Config
from ipc import IPCType, IPC


class Database(object):
    """Objeto que contiene:
    - los paquetes disponibles
    - los paquetes que están servidos
    - donde están servidos los paquetes (dirección y puerto)
    """

    def __init__(self, configuration: Config) -> None:
        self.configuration = configuration
        self.available_packages: list[Sequence[str]] = list()
        self.current_served_packages: dict[str, tuple[str, int]] = {}

    def update(self) -> None:
        """Actualiza los atributos del objeto."""
        pass

    def get_available_packages(self) -> list[Sequence[str]]:
        """Devuelve los paquetes disponibles de la base de datos."""
        self.available_packages.clear()
        with os.scandir(self.configuration.package_dir) as package_dir:
            for entry in package_dir:
                if entry.is_file() and ".tar" in entry.name:
                    self.available_packages.append(entry.name)
        self.available_packages.sort(key=str.lower)  # type: ignore
        return self.available_packages.copy()

    def get_served_packages(self) -> dict[str, tuple[str, int]]:
        """Devuelve los paquetes que están servidos actualmente."""
        self.update()
        return self.current_served_packages

# [GLP3] Copyright (C) 2024  Michel Novus

import os

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
        self.available_packages: list[str] = list()
        self.current_served_packages: dict[str, tuple[str, int]] = {}

    def get_available_packages(self) -> list[str]:
        """Devuelve los paquetes disponibles de la base de datos.

        Se espera que los paquetes no tengan espacios en sus nombres y
        sean archivos tar. Los directorios son ignorados.
        """
        self.available_packages.clear()
        with os.scandir(self.configuration.package_dir) as package_dir:
            for entry in package_dir:
                if (
                    entry.is_file()
                    and ".tar" in entry.name
                    and not " " in entry.name
                ):
                    self.available_packages.append(entry.name)
        self.available_packages.sort(key=str.lower)  # type: ignore
        return self.available_packages.copy()

    def get_served_packages(self) -> dict[str, tuple[str, int]]:
        """Devuelve los paquetes que están servidos actualmente.

        Se comunica por IPC con localdocd, envía el mensaje
        '{"command": "get_served_packages"}' y espera por respuesta
        {"package_name": "addr port", ...}.
        """
        ipc = IPC(self.configuration.socket_filepath, IPCType.CLIENT)
        response = ipc.communicate({"command": "get_served_packages"})
        if response != {"response": "None"}:
            served_packages = {}
            for package_name, web_socket in response.items():
                addr, port = web_socket.split(" ")
                served_packages[package_name] = (addr, int(port))
            self.current_served_packages.clear()
            self.current_served_packages.update(served_packages)
        elif response == {"status": "clear"}:
            self.current_served_packages.clear()
        elif response == {}:
            raise RuntimeError(
                "Ocurrió un error en la comunicación con localdocd."
            )
        return self.current_served_packages

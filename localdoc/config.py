# [GLP3] Copyright (C) 2024  Michel Novus

import unittest
import tomllib
from tomllib import TOMLDecodeError


def new(
    socket: str = "127.0.0.2",
    port: int = 21980,
    package_dir: str = "~/.local/localdoc",
) -> str:
    """Genera la configuración del programa en formato TOML como una cadena."""

    conf: list[str] = list()
    conf.append("# Configuración del programa 'localdoc'.")
    conf.append("# Éste archivo se regenera de vez en cuando, los")
    conf.append("# valores que se hallan asignado en las diferentes")
    conf.append("# secciones se mantendrán, excepto la sección [runtime].")
    conf.append("")
    conf.append("[user]")
    conf.append(f'PACKAGE_DIR = "{package_dir}"')
    conf.append("# En construcción...")
    conf.append("")
    conf.append("# ----------------------------------------------------")
    conf.append("# ------ ¡NO EDITE LA SIGUIENTE SECCIÓN A MANO! ------")
    conf.append("# ----------------------------------------------------")
    conf.append("")
    conf.append("[runtime]")
    conf.append('SOCK_TYPE = "TCP"')
    conf.append(f"SOCK_ADDRESS = {socket}")
    conf.append(f"SOCK_PORT = {port}")
    conf.append("")
    conf.append("# ----------------------------------------------------")
    conf.append("")

    return "\n".join(conf)


class Config(object):
    """Carga la configuración almacenada en una cadena con formato TOML.

    Si la cadena es inválida eleva la excepción TOMLDecodeError.
    """

    def __init__(self, config_data: str) -> None:
        self._data = tomllib.loads(config_data)
        self.socket_type = self._data["runtime"]["SOCK_TYPE"]
        self.socket_addr = self._data["runtime"]["SOCK_ADDRESS"]
        self.socket_port = self._data["runtime"]["SOCK_PORT"]


class _ModuleTests(unittest.TestCase):
    """Tests unitarios de módulo config.py"""

    def test_new(self):
        addr, port = ("127.0.0.2", 1540)
        packages_directory = "/un/directorio/destino"
        expected_data = (
            "# Configuración del programa 'localdoc'.\n"
            "# Éste archivo se regenera de vez en cuando, los\n"
            "# valores que se hallan asignado en las diferentes\n"
            "# secciones se mantendrán, excepto la sección [runtime].\n"
            "\n"
            "[user]\n"
            f'PACKAGE_DIR = "{packages_directory}"\n'
            "# En construcción...\n"
            "\n"
            "# ----------------------------------------------------\n"
            "# ------ ¡NO EDITE LA SIGUIENTE SECCIÓN A MANO! ------\n"
            "# ----------------------------------------------------\n"
            "\n"
            "[runtime]\n"
            'SOCK_TYPE = "TCP"\n'
            f"SOCK_ADDRESS = {addr}\n"
            f"SOCK_PORT = {port}\n"
            "\n"
            "# ----------------------------------------------------\n"
        )
        new_config = new(addr, port, packages_directory)
        self.assertEqual(new_config, expected_data)

    def test_Config(self):
        toml = (
            "[runtime]\n"
            'SOCK_TYPE = "TCP"\n'
            'SOCK_ADDRESS = "127.0.0.3"\n'
            "SOCK_PORT = 1540\n"
        )
        config = Config(toml)
        self.assertEqual(config.socket_type, "TCP")
        self.assertEqual(config.socket_addr, "127.0.0.3")
        self.assertEqual(config.socket_port, 1540)

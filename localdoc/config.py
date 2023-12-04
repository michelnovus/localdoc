# [GLP3] Copyright (C) 2024  Michel Novus
"""Guarda las definiciones del manejador del archivo de configuración."""

import unittest
import tomllib
from tomllib import TOMLDecodeError


def new(
    socket: str = "/tmp/localdoc.socket",
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
    conf.append("")
    conf.append("# Indica dónde se guardarán los paquetes")
    conf.append(f'PACKAGE_DIR = "{package_dir}"')
    conf.append("")
    conf.append("# En construcción...")
    conf.append("")
    conf.append("# ----------------------------------------------------")
    conf.append("# ------ ¡NO EDITE LA SIGUIENTE SECCIÓN A MANO! ------")
    conf.append("# ----------------------------------------------------")
    conf.append("")
    conf.append("[runtime]")
    conf.append('SOCKET_TYPE = "AF_UNIX"')
    conf.append(f'SOCKET_PATH = "{socket}"')
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
        self.socket_type = self._data["runtime"]["SOCKET_TYPE"]
        self.socket_path = self._data["runtime"]["SOCKET_PATH"]

        self.package_dir = self._data["user"]["PACKAGE_DIR"]


class _ModuleTests(unittest.TestCase):
    """Tests unitarios de módulo config.py"""

    def test_new(self):
        path = "/tmp/localdoc.socket"
        packages_directory = "/un/directorio/destino"
        expected_data = (
            "# Configuración del programa 'localdoc'.\n"
            "# Éste archivo se regenera de vez en cuando, los\n"
            "# valores que se hallan asignado en las diferentes\n"
            "# secciones se mantendrán, excepto la sección [runtime].\n"
            "\n"
            "[user]\n"
            "\n"
            "# Indica dónde se guardarán los paquetes\n"
            f'PACKAGE_DIR = "{packages_directory}"\n'
            "\n"
            "# En construcción...\n"
            "\n"
            "# ----------------------------------------------------\n"
            "# ------ ¡NO EDITE LA SIGUIENTE SECCIÓN A MANO! ------\n"
            "# ----------------------------------------------------\n"
            "\n"
            "[runtime]\n"
            'SOCKET_TYPE = "AF_UNIX"\n'
            f'SOCKET_PATH = "{path}"\n'
            "\n"
            "# ----------------------------------------------------\n"
        )
        new_config = new(path, packages_directory)
        self.assertEqual(new_config, expected_data)

    def test_Config(self):
        toml = (
            "[user]\n"
            'PACKAGE_DIR = "/un/directorio"\n'
            "[runtime]\n"
            'SOCKET_TYPE = "AF_UNIX"\n'
            'SOCKET_PATH = "/tmp/localdoc.socket"\n'
        )
        config = Config(toml)
        self.assertEqual(config.socket_type, "AF_UNIX")
        self.assertEqual(config.socket_path, "/tmp/localdoc.socket")

    def test_new_in_Config(self):
        toml = new(socket="/tmp/localdoc.socket", package_dir="/el/dir")
        config = Config(toml)
        self.assertEqual(config.socket_type, "AF_UNIX")
        self.assertEqual(config.socket_path, "/tmp/localdoc.socket")
        self.assertEqual(config.package_dir, "/el/dir")

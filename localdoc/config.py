# [GLP3] Copyright (C) 2024  Michel Novus

import unittest
import tomllib
from tomllib import TOMLDecodeError


def new(socket: str = "127.0.0.2", port: int = 21980) -> str:
    """Genera la configuración predeterminada del programa en
    formato TOML como una cadena."""

    conf: list[str] = list()
    conf.append("# Configuración predeterminada del programa 'localdoc'")
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
        expected_data = (
            "# Configuración predeterminada del programa 'localdoc'\n"
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
        new_config = new(addr, port)
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

# [GLP3] Copyright (C) 2024  Michel Novus
"""Objetos de configuración del programa."""

import tomllib
from typing import NamedTuple


class Config(NamedTuple):
    """Configuración del programa en tiempo de ejecución."""

    socket_type: str
    socket_filepath: str
    package_dir: str


def load_configuration(data: str) -> Config:
    """Carga la configuración guardada en 'data' a un objeto NamedTuple
     llamado 'Config'.

    ### Excepciones:
    - TOMLDecodeError: 'data' inválida
    - KeyError: no se encontró una clave válida
    """

    _data = tomllib.loads(data)
    config = Config(
        socket_type=_data["runtime"]["SOCKET_TYPE"],
        socket_filepath=_data["runtime"]["SOCKET_PATH"],
        package_dir=_data["user"]["PACKAGE_DIR"],
    )
    return config


def generate_new_configuration(
    socket_filepath: str, package_dir: str, socket_type: str = "AF_UNIX"
) -> str:
    """Genera la configuración del programa en formato TOML como una string."""

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
    conf.append("# ----------------------------------------------------")
    conf.append("# ------ ¡NO EDITE LA SIGUIENTE SECCIÓN A MANO! ------")
    conf.append("# ----------------------------------------------------")
    conf.append("")
    conf.append("[runtime]")
    conf.append(f'SOCKET_TYPE = "{socket_type}"')
    conf.append(f'SOCKET_PATH = "{socket_filepath}"')
    conf.append("")
    conf.append("# ----------------------------------------------------")
    conf.append("")
    return "\n".join(conf)

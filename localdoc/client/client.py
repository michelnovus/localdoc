# [GLP3] Copyright (C) 2024  Michel Novus
"""Define la interfáz del cliente."""

import sys

from config import Config
from .console import console, Show
from .arguments import OPT, CMD, arguments
from .execution_tasks import List, Serve


def run_client(configuration: Config) -> None:
    """Ejecuta el proceso interactivo."""
    args = arguments()
    if len(args) == 0:
        sys.exit(1)

    match args["CMD"]:
        case CMD.LIST:
            List(configuration, [], []).exec()
        case CMD.SERVE:
            if len(args["ARG"]) == 0:
                console.print("No hay argumentos!")
                sys.exit(1)
            if OPT.OPEN in args["OPT"]:
                open_in_browser = [OPT.OPEN]
            else:
                open_in_browser = []
            Serve(configuration, args["ARG"], open_in_browser).exec()
        case _:
            # DEBUG
            console.print(Show.warn("comando en construcción!"))
            sys.exit(0)

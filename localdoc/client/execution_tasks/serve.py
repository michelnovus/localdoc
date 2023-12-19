# [GLP3] Copyright (C) 2024  Michel Novus

import ipc
from client.console import console, Show


def serve_package(ipc_socket: str, package: str) -> tuple[str, int]:
    """Comunica a 'localdocd' para que intente servir package."""
    response = ipc.communicate(ipc_socket, {"serve_package": package})
    if response["response"] == "package is already served":
        msg = Show.warn(f"[b]{package}[/b] ya esta servido en:")
        msg += "\n"
        msg += Show.space("")
        msg += f"[b]http://{response['addr']}:{response['port']}[/b]"
        console.print(msg)
        return (response["addr"], response["port"])
    elif response["response"] == "package is served":
        msg = Show.info(f"[b]{package}[/b] servido en:")
        msg += "\n"
        msg += Show.space("")
        msg += f"[b]http://{response['addr']}:{response['port']}[/b]"
        console.print(msg)
        return (response["addr"], response["port"])
    else:
        raise RuntimeError("Error indeterminado.")  # TEMP

# [GLP3] Copyright (C) 2024  Michel Novus

import ipc_purepy as ipc


def close_daemon(ipc_socket: str) -> None:
    """Cierra el daemon activo."""
    response = ipc.communicate(ipc_socket, "close_daemon")

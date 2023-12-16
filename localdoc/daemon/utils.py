# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path

import ipc


def clear_tempdir(socket_dir: str) -> None:
    """Limpia el directorio /tmp de rastros de antiguos procesos de localdoc."""
    if socket_dir.startswith("/tmp/localdoc"):
        try:
            os.rmdir(socket_dir)
        except FileNotFoundError:
            pass
        except OSError:
            pass


def is_localdocd_running(ipc_socket: str) -> bool:
    """Comprueba si el daemon esta ejecutandose."""
    try:
        response = ipc.communicate(ipc_socket, {"localdocd_status": None})
    except FileNotFoundError:
        return False
    except ConnectionRefusedError:
        return False
    if response["localdocd_status"] == "up":
        return True
    else:
        return False

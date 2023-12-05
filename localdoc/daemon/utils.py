# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path

from ipc import IPCType, IPC


def clear_tempdir(socket_dir: str) -> None:
    """Limpia el directorio /tmp de rastros de antiguos procesos de localdoc."""
    if socket_dir.startswith("/tmp/localdoc"):
        try:
            os.rmdir(os.path.split(socket_dir)[0])
        except FileNotFoundError:
            pass
        except OSError:
            pass


def localdocd_is_running(socket_path: str) -> bool:
    """Comprueba si el daemon esta ejecutandose."""
    try:
        ipc = IPC(socket_path, IPCType.CLIENT)
        response = ipc.communicate({"localdocd_status": ""})
    except FileNotFoundError:
        return False
    except ConnectionRefusedError:
        return False
    if response["localdocd_status"] == "alive":
        return True
    else:
        return False

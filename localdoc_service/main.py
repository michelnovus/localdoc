# [MIT License] Copyright (c) 2024 Michel Novus
"""Main execution module."""

import os
import os.path
import time
import socketserver

import debug
from server import DispatcherHandler


def main():
    debug.print("Cargando Localdoc Service.")
    LOCALDOC_RUNTIME_DIR = f"/run/user/{os.getuid()}/localdoc"
    SOCKET = os.path.join(LOCALDOC_RUNTIME_DIR, "service.socket")
    os.makedirs(LOCALDOC_RUNTIME_DIR, exist_ok=True)
    debug.print(f"Socket en: {SOCKET}")

    debug.print("Iniciando servidor.")
    with socketserver.UnixStreamServer(SOCKET, DispatcherHandler) as unix_sock:
        try:
            unix_sock.serve_forever()
        except KeyboardInterrupt:
            debug.print("Cerrando servidor.")
            pass
        finally:
            time.sleep(0.5)
            debug.print("Eliminando Unix socket.")
            os.unlink(SOCKET)
    debug.print("Terminado.")


if __name__ == "__main__":
    main()

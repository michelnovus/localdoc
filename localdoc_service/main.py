# [MIT License] Copyright (c) 2024 Michel Novus
"""Main execution module."""

import os
import os.path
import time
import socketserver
from server import DispatcherHandler


def main():
    LOCALDOC_RUNTIME_DIR = f"/run/user/{os.getuid()}/localdoc"
    SOCKET = os.path.join(LOCALDOC_RUNTIME_DIR, "localdoc.socket")

    os.makedirs(LOCALDOC_RUNTIME_DIR, exist_ok=True)
    with socketserver.UnixStreamServer(SOCKET, DispatcherHandler) as unix_sock:
        try:
            unix_sock.serve_forever()
        except KeyboardInterrupt:
            pass
        finally:
            time.sleep(1)
            os.unlink(SOCKET)


if __name__ == "__main__":
    main()

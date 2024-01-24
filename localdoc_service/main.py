# [MIT License] Copyright (c) 2024 Michel Novus
"""Main execution module."""

import os
import os.path
import time
import socketserver

from journal import Journal
from handler import ConnectionHandler
from service import Config


def main():
    journal = Journal()
    journal.info("Launch Localdoc service.")
    config = Config(
        runtime_directory=f"/run/user/{os.getuid()}/localdoc",
        socket_name="service.socket",
    )
    os.makedirs(config.runtime_directory, exist_ok=True)
    journal.info(f"Created runtime in: {config.runtime_directory}")
    journal.info("Init socket.")
    with socketserver.UnixStreamServer(
        config.socket_path, ConnectionHandler
    ) as unix_sock:
        try:
            unix_sock.serve_forever()
        except KeyboardInterrupt:
            journal.info("Closing socket.")
            pass
        finally:
            time.sleep(0.5)
            journal.info("Unlink socket.")
            os.unlink(config.socket_path)
    journal.info("Localdoc service terminated.")


if __name__ == "__main__":
    main()

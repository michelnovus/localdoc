# [GLP3] Copyright (C) 2024  Michel Novus

import os.path

from initializer import (
    mk_main_configuration_directory,
    mk_main_configuration_file,
    load_configuration_with_notices,
    init_daemon,
)
from daemon import is_localdocd_running
from client import console, Show, run_client


def main() -> None:
    mk_main_configuration_directory()
    mk_main_configuration_file()
    configuration = load_configuration_with_notices()
    if not (
        is_localdocd_running(configuration.socket_filepath)
        or os.path.exists(configuration.socket_filepath)
    ):
        console.print(Show.warn("No esta cargado el proceso [b]localdocd[/]."))
        init_daemon(configuration.socket_filepath)
        configuration = load_configuration_with_notices(exit_on_error=False)
    run_client(configuration)


if __name__ == "__main__":
    main()

# [GLP3] Copyright (C) 2024  Michel Novus

import os.path
from rich.text import Text
from rich.table import Table

from packages import get_available_packages, get_served_packages
from client.console import console
from .base_class import BaseExecClass


class List(BaseExecClass):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        console.print("[b]Listado de paquetes disponibles:[/]")
        table = Table(show_header=False, box=None)
        table.add_column("package_name", justify="left", max_width=30)
        table.add_column("served_in", justify="left")
        served_packages = get_served_packages(
            self.configuration.socket_filepath
        )
        for package in get_available_packages(self.configuration.package_dir):
            package_name = Text(
                text=f"  {os.path.split(package)[1]}",
                style="bold",
                overflow="ellipsis",
            )
            if package in served_packages.keys():
                served_in = Text(
                    "[cyan]:arrow_right:[/]"
                    f"http://{served_packages[package][0]}/"
                    f"{served_packages[package][1]}",
                    style="italic",
                )
                table.add_row(package_name, served_in)
            else:
                table.add_row(package_name)
        console.print(table)
        console.print()

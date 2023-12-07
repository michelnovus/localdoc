# [GLP3] Copyright (C) 2024  Michel Novus

from rich.text import Text
from rich.table import Table

from client.console import console
from database import Database
from .base_class import BaseExecClass


class List(BaseExecClass):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.database = Database(self.configuration)
        self.available_packages = self.database.get_available_packages()
        self.served_packages = self.database.get_served_packages()

    def exec(self) -> None:
        console.print("[b]Listado de paquetes disponibles:[/]")
        table = Table(show_header=False, box=None)
        table.add_column("package_name", justify="left", max_width=30)
        table.add_column("served_in", justify="left")
        for package in self.available_packages:
            package_name = Text(
                text=f"  {package[0 : package.find('.tar')]}",
                style="bold",
                overflow="ellipsis",
            )
            if package in self.served_packages.keys():
                served_in = Text(
                    "[cyan]:arrow_right:[/]"
                    f"http://{self.served_packages[package][0]}/"
                    f"{self.served_packages[package][1]}",
                    style="italic",
                )
                table.add_row(package_name, served_in)
            else:
                table.add_row(package_name)
        console.print(table)
        console.print()

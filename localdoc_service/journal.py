# [MIT License] Copyright (c) 2024 Michel Novus

import rich
from rich.text import Text


class Journal(object):
    def info(self, text: str) -> None:
        """Print info string in stdout."""
        rich.print(Text("[info]", style="yellow bold"), text)

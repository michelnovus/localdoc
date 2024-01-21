# [MIT License] Copyright (c) 2024 Michel Novus

# DEBUG

import rich


def print(text: str) -> None:
    rich.print(f"[yellow]\[debug][/] {text}")

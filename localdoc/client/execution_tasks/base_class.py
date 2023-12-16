# [GLP3] Copyright (C) 2024  Michel Novus

from typing import Sequence

from configuration import Config
from client.arguments import OPT


class BaseExecClass(object):
    def __init__(
        self,
        configuration: Config,
        args: list[Sequence[str]],
        options: list[Sequence[OPT]],
    ) -> None:
        self.configuration = configuration
        self.args = args
        self.options = options

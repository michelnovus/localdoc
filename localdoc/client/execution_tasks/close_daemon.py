# [GLP3] Copyright (C) 2024  Michel Novus

from .base_class import BaseExecClass


class CloseDaemon(BaseExecClass):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

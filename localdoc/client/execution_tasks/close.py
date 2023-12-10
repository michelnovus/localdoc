# [GLP3] Copyright (C) 2024  Michel Novus

from ipc import IPC, IPCType
from .base_class import BaseExecClass


class Close(BaseExecClass):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

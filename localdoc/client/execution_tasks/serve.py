# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path
import shutil
import sys
import threading
import tarfile

from typing import Union, Sequence
from ipc import IPC, IPCType
from .base_class import BaseExecClass


class Serve(BaseExecClass):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path
import shutil
import sys
import tarfile

from .base_class import BaseExecClass


class Install(BaseExecClass):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)

# [GLP3] Copyright (C) 2024  Michel Novus
"""Proceso localdocd no interactivo."""

from .launcher import daemonize
from ..config import Config
from .utils import clear_tempdir, localdocd_is_running

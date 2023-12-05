# [GLP3] Copyright (C) 2024  Michel Novus

import os
import os.path

# -------------- [Constantes predeterminadas] --------------
CONFIG_DIRPATH = os.path.join(
    os.getenv("HOME", os.getcwd()), ".config/localdoc"
)
CONFIG_FILEPATH = os.path.join(CONFIG_DIRPATH, "localdoc.toml")
PACKAGE_DIRECTORY = os.path.join(
    os.getenv("HOME", os.getcwd()), ".local/localdoc/packages"
)
# -----------------------------------------------------------

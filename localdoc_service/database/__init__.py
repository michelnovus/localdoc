# [MIT License] Copyright (c) 2024 Michel Novus

import os.path
import sqlite3


class Database(object):
    """Represents the database and allowing queries to this."""

    def __init__(self, filepath: str) -> None:
        self.database_path = filepath

    def create_db(self) -> None:
        """Creates a new database file, raise OSError if already exist."""
        if os.path.exists(self.database_path):
            raise OSError(f"{self.database_path} is already exist.")
        con = sqlite3.connect(self.database_path)
        cur = con.cursor()
        with open(
            os.path.join(os.path.split(__file__)[0], "tables.sql")
        ) as tables_file:
            tables = tables_file.read()
        cur.executescript(tables)
        con.commit()
        con.close()

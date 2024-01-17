# [MIT License] Copyright (c) 2024 Michel Novus

from enum import StrEnum


class Command(StrEnum):
    """Tasks that the process can execute."""

    INSERT = "INSERT"  # Insert documentation into database
    DELETE = "DELETE"  # Delete documentation from database
    UPDATE = "UPDATE"  # Modify documentation into database
    LAUNCH = "LAUNCH"  # Serve documentation to consult
    DETAIN = "DETAIN"  # Stop served documentation
    STATUS = "STATUS"  # Get current state of process

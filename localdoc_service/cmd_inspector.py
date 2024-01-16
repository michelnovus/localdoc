# [MIT License] Copyright (c) 2024 Michel Novus


class Inspector(object):
    """The object analyzes whether the string containing
    a command provided by check method is valid.
    """

    def __init__(self) -> None:
        self.commands = {}

    def add_command(self, name: str, values_amount: int) -> None:
        """Add a new command to Inspector.
        values_amount is the number of arguments needed in the
        command, it must be a positive integer, or -1 if they are an
        undetermined amount.
        """
        self.commands[name] = values_amount

    def is_valid(self, cmd: str) -> bool:
        """Check if a valid command.
        Return True if valid, False otherwise.
        """
        command_separator_index = cmd.strip().find(" ")
        if command_separator_index == -1:
            return False
        command = cmd[0:command_separator_index].strip()
        values = cmd[command_separator_index:].split()
        amount = len(values)
        if command not in list(self.commands.keys()):
            return False
        if self.commands[command] != -1 and amount != self.commands[command]:
            return False
        return True

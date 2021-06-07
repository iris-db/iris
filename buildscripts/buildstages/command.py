import subprocess
from dataclasses import dataclass
from typing import Dict, Any


@dataclass
class Command:
    """Runs a command in the terminal. Commands must be in the format of `root sub0 sub1 --flag1 -flag2`"""

    _cmd_str: str

    @classmethod
    def build(cls, name: str, sub: list[str] = None, flags: Dict[str, Any] = None):
        """Creates a command from a root command name, a list of sub commands, and flags. Useful for building a
        a command string dynamically."""
        cmd_str = f"{name} "

        for s in sub:
            cmd_str += f"{s} "

        if flags is not None:
            for key, value in flags.items():
                cmd_str += key + " " + str(value)

        return cls(cmd_str)

    def to_string(self):
        """Command string representation."""
        return self._cmd_str

    def exec(self) -> int:
        """Executes the command, returning the exit code."""
        res = subprocess.run(self._cmd_str)
        return res.returncode


@dataclass
class BuildError(Exception):
    command_name: str

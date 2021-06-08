import subprocess
from dataclasses import dataclass


@dataclass
class Command:
    """Runs a command in the terminal. Commands must be in the format of `root sub0 sub1 --flag1 -flag2`"""
    _string: str

    @property
    def string(self):
        return self._string

    def exec(self) -> int:
        """Executes the command, returning its exit code."""
        result = subprocess.run(self._string, shell=True)
        return result.returncode


@dataclass
class BuildError(Exception):
    command_name: str

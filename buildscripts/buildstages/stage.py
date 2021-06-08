from dataclasses import dataclass
from typing import Dict, Any


@dataclass
class FlagSet:
    """Provides a set of utility methods for a flag dictionary."""
    _flags: Dict[str, Any]

    def get(self, *names):
        """Gets the first flag value by one of its aliases. Example: `get('-a', '--all')"""
        names = list(names)
        names.sort(key=len)

        for arg in names:
            if arg in self._flags:
                return self._flags[arg]

        return None

    @property
    def flags(self):
        return self._flags


class BuildStage:
    """Represents a CLI build stage"""
    name: str
    working_directory: str

    def run(self, flags: FlagSet):
        """Runs the build stage, returning the ProcessResult."""
        pass


def stage_dict(stages: list[BuildStage]) -> dict[str, BuildStage]:
    """Converts a list of build stages into a dict mapping the name of the stage as the key to its instance."""
    new = {}

    for stage in stages:
        new[stage.name] = stage

    return new

from dataclasses import dataclass
from typing import Dict, Any


@dataclass
class FlagSet:
    """Provides a set of utility methods for a flag dictionary."""
    _flags: Dict[str, Any]

    @classmethod
    def from_list(cls, ls: list[str]):
        def is_flag(obj) -> bool:
            return isinstance(obj, str) and obj[0] == '-'

        ls_len = len(ls)
        new = {}

        for i in range(ls_len):
            item = ls[i]
            if is_flag(item):
                new[item] = None
                next_index = i + 1
                if next_index < ls_len:
                    next_item = ls[next_index]
                    if not is_flag(next_item):
                        new[item] = next_item

        return cls(new)

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


class Stage:
    """Represents a CLI build stage"""
    name: str
    working_directory: str

    def run(self, flags: FlagSet):
        """Runs the build stage, returning the ProcessResult."""
        pass


def stage_dict(stages: list[Stage]) -> dict[str, Stage]:
    """Converts a list of build stages into a dict mapping the name of the stage as the key to its instance."""
    new = {}

    for stage in stages:
        new[stage.name] = stage

    return new

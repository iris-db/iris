from typing import Dict, Any

from buildstages.command import Command
from buildstages.stage import BuildStage


class Binary(BuildStage):
    name = "server"

    def run(self, flags: Dict[str, Any]):
        pass


_BUILD_STAGES = [
    Binary()
]


def _make_build_stages():
    """Converts a list of build stages into a dict mapping the name of the stage as the key to its instance."""
    new = {}

    for stage in _BUILD_STAGES:
        new[stage.name] = stage

    return new


BUILD_STAGES = _make_build_stages()

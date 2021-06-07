from typing import Dict, Any, Optional


class BuildStage:
    """Represents a CLI build stage"""
    name: str

    def run(self, flags: Dict[str, Any]):
        """Runs the build stage, returning the ProcessResult."""
        pass

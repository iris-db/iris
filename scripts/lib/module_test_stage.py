from lib.stage import Stage, FlagSet
from shutil import which


class TestStage(Stage):
    """A stage that executes the tests for each module."""
    required_path_commands: list[str]
    #
    # def run(self, flags: FlagSet):
    #     """Verify the require commands exist."""
    #     for cmd in self.required_path_commands:
    #         is_cmd = which(cmd) is not None
    #         if not is_cmd:
    #             pass

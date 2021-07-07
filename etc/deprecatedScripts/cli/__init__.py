import sys
from typing import Dict

from cli.utils import directory_count, DirectoryChangeResult, CLI_MESSANGER
from lib.command import Command
from lib.stage import Stage, FlagSet


def main(stage_dict):
    """Runs the CLI for a stage dict."""
    process = ProcessRunner()
    process.run_stage(stage_dict)


class ProcessRunner:
    """Executes processes in the proper directories."""
    _has_exited_initial_directory = False

    def __init__(self, sys_argv: list[str] = None):
        if sys_argv is None:
            self._sys_argv = sys.argv
        else:
            self._sys_argv = sys_argv

    @property
    def sys_args(self):
        return self._sys_argv

    def run_stage(self, stage_map: Dict[str, Stage]):
        """Runs a Stage based on a dictionary of Stages mapping the stage name (str) to the Stage (Stage)."""
        try:
            stage = stage_map[self.get_target()]
            working_directory = stage.working_directory

            self.enter_directory(working_directory)

            stage.run(flags=FlagSet.from_list(self.sys_args[2:]))

            self.enter_root_directory(working_directory)
        except KeyError:
            CLI_MESSANGER.display_invalid_target_error()

    def get_target(self) -> str:
        """Get the word right after the script file name. Example: python3 build.py the_target"""
        args = self._sys_argv[1:]
        index = 1
        return args[index] if index < len(args) else None

    def enter_directory(self, directory: str) -> DirectoryChangeResult:
        """Change to the specified directory relative to the project root. The current directory must be at the
        project root."""
        res = DirectoryChangeResult()

        if not self._has_exited_initial_directory:
            Command("cd ..").exec()
            res.increment_dir_out()

            self._has_exited_initial_directory = True

        Command(f"cd {directory}").exec()
        res.increment_dir_in(directory_count(directory))

        return res

    @staticmethod
    def enter_root_directory(current_directory: str) -> DirectoryChangeResult:
        """Enters the root repository directory."""
        res = DirectoryChangeResult()

        back_cd = ""
        for i in range(directory_count(current_directory)):
            back_cd += "../"

        Command(f"cd {back_cd}").exec()
        return res

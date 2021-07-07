from lib.command import Command
from lib.stage import Stage, stage_dict, FlagSet


class Binary(Stage):
    name = "server"
    working_directory = "source/server"

    def run(self, flags: FlagSet):
        build_cmd = "cargo build"

        is_local = flags.get("-d", "--dev")
        if is_local is None:
            build_cmd += " --release"

        Command(build_cmd).exec()


BUILD_STAGES = stage_dict([
    Binary()
])

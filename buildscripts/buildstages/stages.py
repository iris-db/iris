from buildstages.command import Command
from buildstages.stage import BuildStage, stage_dict, FlagSet


class Binary(BuildStage):
    name = "server"
    working_directory = "source/server"

    def run(self, flags: FlagSet):
        build_cmd = "cargo build"

        is_local = flags.get("-l", "--local")
        if not is_local:
            build_cmd += " --release"

        Command(build_cmd).exec()


BUILD_STAGES = stage_dict([
    Binary()
])

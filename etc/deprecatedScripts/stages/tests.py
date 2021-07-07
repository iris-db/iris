from lib.command import Command
from lib.module_test_stage import TestStage
from lib.stage import FlagSet, stage_dict


class ServerTest(TestStage):
    name = "IrisDB-Server"
    working_directory = "source/server"
    required_path_commands = [
        "cargo",
        "rustup"
    ]

    def run(self, flags: FlagSet):
        super().run(flags)

        Command("cargo +nightly test").exec()


TEST_STAGES = stage_dict([
    ServerTest()
])

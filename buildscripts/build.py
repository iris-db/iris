import sys

from buildstages.command import Command
from buildstages.stage import FlagSet
from buildstages.stages import BUILD_STAGES
from cli_utils import MessageUtils, directory_count


def main():
    args = sys.argv[1:]

    if len(args) == 0:
        MessageUtils.display_help_msg()
        exit(1)

    target = args[0]
    flags = args[1:]

    try:
        build_stage = BUILD_STAGES[target]

        Command(f"cd ../{build_stage.working_directory}").exec()

        build_stage.run(FlagSet.from_list(flags))

        back_cd = ""
        for i in range(directory_count(build_stage.working_directory)):
            back_cd += "../"

        Command(f"cd {back_cd}").exec()
    except KeyError:
        MessageUtils.display_invalid_target_error()
        exit(1)


if __name__ == "__main__":
    main()

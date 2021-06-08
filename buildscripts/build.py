import sys

from buildstages.command import Command
from buildstages.stages import BUILD_STAGES
from cli_utils import MessageUtils


def main():
    args = sys.argv[1:]

    if len(args) == 0:
        MessageUtils.display_help_msg()
        exit(1)

    target = args[0]
    flags = args[1:]

    try:
        build_stage = BUILD_STAGES[target]

        Command(f"cd ../{build_stage.working_directory}")

        build_stage.run(flags)
    except KeyError:
        MessageUtils.display_invalid_target_error()
        exit(1)


if __name__ == "__main__":
    main()

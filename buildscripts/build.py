import sys

from buildstages.stages import BUILD_STAGES


def main():
    args = sys.argv[1:]

    if len(args) == 0:
        _MessageUtils.display_help_msg()
        return

    target = args[0]
    flags = args[1:]

    try:
        build_stage = BUILD_STAGES[target]
        build_stage.run(flags)
    except KeyError:
        _MessageUtils.display_invalid_target_error()
        return


class _MessageUtils:
    """Utility class for printing common CLI messages."""

    def __init__(self):
        raise TypeError("Cannot create an instance of MessageUtils")

    @staticmethod
    def display_help_msg():
        print("Help!")

    @staticmethod
    def display_invalid_target_error():
        print("Invalid target.")


if __name__ == "__main__":
    main()

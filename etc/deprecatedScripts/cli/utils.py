from dataclasses import dataclass


@dataclass
class CliMessanger:
    """Class for printing common CLI messages."""
    help_msg: str
    invalid_target_msg: str

    def display_help_msg(self):
        print(self.help_msg)

    def display_invalid_target_error(self):
        print(self.invalid_target_msg)


CLI_MESSANGER = CliMessanger(
    help_msg="Help!",
    invalid_target_msg="This target does not exist!"
)


class DirectoryChangeResult:
    """Container class for holding the result from multiple cd's executed in sequence."""
    def __init__(self, dir_out: int = 0, dir_in: int = 0):
        self._dir_out = dir_out
        self._dir_in = dir_in

    @property
    def dir_in(self):
        return self._dir_in

    @property
    def dir_out(self):
        return self._dir_out

    def net_dir_change(self):
        """The total amount of directory changes."""
        return self.dir_in + self.dir_out

    def increment_dir_out(self, amt: int = 1):
        """Increments dir out by one."""
        self._dir_out += amt

    def increment_dir_in(self, amt: int = 1):
        """Increments dir in by one."""
        self._dir_in += amt


def directory_count(directory: str) -> int:
    """Gets the number of directories in a directory string."""
    count = 1

    for c in directory:
        if c == '/':
            count += 1

    if directory[len(directory) - 1] == '/':
        count -= 1

    return count

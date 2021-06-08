class MessageUtils:
    """Utility class for printing common CLI messages."""
    def __init__(self):
        raise TypeError("Cannot create an instance of MessageUtils")

    @staticmethod
    def display_help_msg():
        print("Help!")

    @staticmethod
    def display_invalid_target_error():
        print("Invalid target.")


def directory_count(directory: str) -> int:
    """Gets the number of directories in a directory string."""
    count = 1

    for c in directory:
        if c == '/':
            count += 1

    if directory[len(directory) - 1] == '/':
        count -= 1

    return count

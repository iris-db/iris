import unittest

from cli_utils import directory_count


class FunctionTests(unittest.TestCase):
    def test_directory_count(self):
        no_trailing_slash_dir = "my/amazing/sub/directories"
        self.assertEqual(4, directory_count(no_trailing_slash_dir))

        trailing_slash_dir = "my/amazing/sub/directories/"
        self.assertEqual(4, directory_count(trailing_slash_dir))


if __name__ == "__main__":
    unittest.main()

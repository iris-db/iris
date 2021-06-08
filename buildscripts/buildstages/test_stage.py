import unittest

from buildstages.stage import FlagSet


class FlagSetTest(unittest.TestCase):
    def test_from_list(self):
        flag_set = FlagSet.from_list(["-Quiet", "-FileName", "SomeFile.txt", "-Output", "Outfile.txt"])

        self.assertEqual({
            "-Quiet": "",
            "-FileName": "SomeFile.txt",
            "-Output": "Outfile.txt"
        }, flag_set.flags)

    def test_get(self):
        alias_first = FlagSet({
            "-f": "alias.txt",
            "--file": "long.txt"
        })
        self.assertEqual("alias.txt", alias_first.get("--file", "-f"))

        only_alias = FlagSet({
            "-f": "alias.txt",
        })
        self.assertEqual("alias.txt", only_alias.get("--file", "-f"))

        only_long = FlagSet({
            "--file": "long.txt",
        })
        self.assertEqual("long.txt", only_long.get("--file", "-f"))

        no_flags = FlagSet({})
        self.assertEqual(None, no_flags.get("--file", "-f"))


if __name__ == "__main__":
    unittest.main()

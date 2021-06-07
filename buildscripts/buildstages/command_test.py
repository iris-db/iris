import unittest

from buildstages.command import Command


class CommandTest(unittest.TestCase):
    def test_build(self):
        cmd = Command.build(
            name="build",
            sub=["server", "fast"],
            flags={
                "--doSomething": "",
                "-Count": 0,
            }
        )

        self.assertEqual("build server fast --doSomething -Count 0", cmd.to_string())


if __name__ == "__main__":
    unittest.main()

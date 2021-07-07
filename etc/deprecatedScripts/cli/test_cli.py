import unittest

from cli import ProcessRunner
from cli.utils import DirectoryChangeResult


class ProcessRunnerTest(unittest.TestCase):
    def test_get_target(self):
        process_with_target = ProcessRunner(sys_argv=[
            "python3",
            "build.py",
            "my_target"
        ])
        self.assertEqual(process_with_target.get_target(), "my_target")

        process_without_target = ProcessRunner(sys_argv=[
            "python3",
            "build.py"
        ])
        self.assertEqual(process_without_target.get_target(), None)


class DirectoryChangeResultTest(unittest.TestCase):
    def test_net_dir_change(self):
        res = DirectoryChangeResult(
            dir_in=12,
            dir_out=13
        )

        self.assertEqual(res.net_dir_change(), 25)

    def test_dir_out_increment(self):
        res = DirectoryChangeResult()

        res.increment_dir_out()
        self.assertEqual(res.dir_out, 1)

        res.increment_dir_out(14)
        self.assertEqual(res.dir_out, 15)

    def test_dir_in_increment(self):
        res = DirectoryChangeResult()

        res.increment_dir_in()
        self.assertEqual(res.dir_in, 1)

        res.increment_dir_in(14)
        self.assertEqual(res.dir_in, 15)


if __name__ == "__main__":
    unittest.main()

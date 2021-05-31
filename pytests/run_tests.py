import importlib
import pathlib
from os import listdir
from os.path import isfile, join

TESTS_MODULE = "tests"

REQUIRED_ATTRIBUTES = {
    "NAME": "constant",
    "test": "function"
}
OPTIONAL_ATTRIBUTES = {
    "before": "function",
    "after": "function"
}


def main():
    """
    Runs all tests in the python files at the TESTS_MODULE.
    """
    full_test_path = join(pathlib.Path(__file__).parent.absolute(), TESTS_MODULE)

    files = [f for f in listdir(full_test_path) if isfile(join(full_test_path, f))]

    for f in files:
        mod = importlib.import_module(f"{TESTS_MODULE}.{str(f[:-3])}")
        attr, has_attr = curry_mod_attr(mod)

        missing_attrs = []

        for a in REQUIRED_ATTRIBUTES:
            if not has_attr(a):
                missing_attrs.append(a)

        if len(missing_attrs) != 0:
            print(f"MALFORMED TEST IN {f}; Missing:")
            for a in missing_attrs:
                print(f"  {a} ({REQUIRED_ATTRIBUTES[a]})")

            print("PLEASE FIX!")
            exit(1)

        # Test arguments
        name = attr("NAME")

        # Test functions
        exec_test = attr("test")

        # Run test
        print(f"TEST | {name}")
        print("=================================================")

        exec_test()

        print("=================================================")


def curry_mod_attr(module):
    """
    Curried functions for getattr and hasattr.
    :param module: The module to use for attribute getting and checking
    :return: A tuple containing getattr at pos 0 and hasattr at pos 1
    """

    def getattr_fn(attr):
        return getattr(module, attr)

    def hasattr_fn(attr):
        return hasattr(module, attr)

    return getattr_fn, hasattr_fn


if __name__ == "__main__":
    main()

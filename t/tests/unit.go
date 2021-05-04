package tests

import (
	"errors"
	"fmt"
	"github.com/iris-db/iris/t/lib"
	"os"
	"strings"
)

type unitTest struct {
	Dir              string                  // Dir is the directory relative to the root project path.
	Exec             func(path string) error // Exec executes the testing processes.
	RequiredCommands []*lib.RequiredCommand  // RequiredCommands are the commands that are required to run the unit tests.
}

var (
	unitTests = []unitTest{
		{
			Dir: "source",
			Exec: func(path string) error {
				fmt.Println("Building project")
				lib.StreamCmd("cargo", "+nightly", "build")

				fmt.Println("Running tests")
				lib.StreamCmd("cargo", "+nightly", "test")

				return nil
			},
			RequiredCommands: []*lib.RequiredCommand{
				lib.NewRequiredCommand("cargo"),
				lib.NewRequiredCommand("rustup", lib.WithValidation(lib.CommandValidator{
					Error: errors.New("nightly toolchain is not installed. Please install it with: rustup install nightly"),
					Validate: func(cmd string) bool {
						toolchains := lib.ExecCmdStdout(cmd, "toolchain", "list")
						return strings.Contains(toolchains, "nightly")
					},
				})),
			},
		},
		{
			Dir: "bson",
			Exec: func(path string) error {
				lib.StreamCmd("go", "test", "-v")
				return nil
			},
			RequiredCommands: []*lib.RequiredCommand{
				lib.NewRequiredCommand("go"),
			},
		},
	}
)

// ExecUnitTests executes all unit tests in the specified directories.
func ExecUnitTests() {
	var testErrors []string

	for _, t := range unitTests {
		for _, c := range t.RequiredCommands {
			if err := c.Validate(); err != nil {
				fmt.Println(err)
				os.Exit(1)
			}
		}

		lib.PrintDivider()
		fmt.Printf("Running [%s] tests\n", t.Dir)
		lib.PrintDivider()

		path := fmt.Sprintf("../%s", t.Dir)

		err := os.Chdir(path)
		if err != nil {
			panic(err)
		}
		if err := t.Exec(path); err != nil {
			testErrors = append(testErrors, path)
		}

		lib.PrintDivider()
		fmt.Printf("Completed [%s] tests\n", t.Dir)
		lib.PrintDivider()

		err = os.Chdir("../t")
		if err != nil {
			panic(err)
		}
	}

	if testErrors != nil {
		for _, p := range testErrors {
			fmt.Printf("[%s] tests did not all pass successfully", p)
		}
		lib.ExitErr(errors.New("failing unit tests"))
	}

	fmt.Println("Unit tests successful")
}

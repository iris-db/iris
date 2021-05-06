package tests

import (
	"errors"
	"fmt"
	"github.com/iris-db/iris/x"
	"os"
	"strings"
)

type unitTest struct {
	Dir              string                  // Dir is the directory relative to the root project path.
	Exec             func(path string) error // Exec executes the testing processes.
	RequiredCommands []*x.RequiredCommand    // RequiredCommands are the commands that are required to run the unit tests.
}

var (
	unitTests = []unitTest{
		{
			Dir: "iris",
			Exec: func(path string) error {
				fmt.Println("Building project")
				x.StreamCmd("cargo", "+nightly", "build")

				fmt.Println("Running tests")
				x.StreamCmd("cargo", "+nightly", "test")

				return nil
			},
			RequiredCommands: []*x.RequiredCommand{
				x.NewRequiredCommand("cargo"),
				x.NewRequiredCommand("rustup", x.WithValidation(x.CommandValidator{
					Error: errors.New("nightly toolchain is not installed. Please install it with: rustup install nightly"),
					Validate: func(cmd string) bool {
						toolchains := x.ExecCmdStdout(cmd, "toolchain", "list")
						return strings.Contains(toolchains, "nightly")
					},
				})),
			},
		},
		{
			Dir: "bsonDeserializer",
			Exec: func(path string) error {
				x.StreamCmd("go", "test", "-v")
				return nil
			},
			RequiredCommands: []*x.RequiredCommand{
				x.NewRequiredCommand("go"),
			},
		},
		{
			Dir: "shell",
			Exec: func(path string) error {
				x.StreamCmd("go", "test", "-v", "./view")
				return nil
			},
			RequiredCommands: []*x.RequiredCommand{
				x.NewRequiredCommand("go"),
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

		printDivider()
		fmt.Printf("Running [%s] tests\n", t.Dir)
		printDivider()

		path := fmt.Sprintf("../%s", t.Dir)

		err := os.Chdir(path)
		if err != nil {
			panic(err)
		}
		if err := t.Exec(path); err != nil {
			testErrors = append(testErrors, path)
		}

		printDivider()
		fmt.Printf("Completed [%s] tests\n", t.Dir)
		printDivider()

		err = os.Chdir("../t")
		if err != nil {
			panic(err)
		}
	}

	if testErrors != nil {
		for _, p := range testErrors {
			fmt.Printf("[%s] tests did not all pass successfully", p)
		}
		x.ExitErr(errors.New("failing unit tests"))
	}

	fmt.Println("Unit tests successful")
}

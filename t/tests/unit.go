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
				cargoManifestPath := fmt.Sprintf("%s/%s", path, "Cargo.toml")

				if err := lib.StreamCmd("cargo", "+nightly", "build", "--manifest-path", cargoManifestPath); err != nil {
					return err
				}
				if err := lib.StreamCmd("cargo", "+nightly", "test", "--manifest-path", cargoManifestPath); err != nil {
					return err
				}

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
	}
)

// ExecUnitTests executes all unit tests in the specified directories.
func ExecUnitTests() {
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
		if err := t.Exec(path); err != nil {
			lib.PrintErrMessage(err.Error())
		}

		lib.PrintDivider()
		fmt.Printf("Completed [%s] tests\n", t.Dir)
		lib.PrintDivider()
	}
}

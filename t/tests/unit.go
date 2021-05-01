package tests

import (
	"fmt"
	"github.com/iris-db/iris/t/lib"
	"os"
	"os/exec"
)

type unitTest struct {
	Dir             string
	RequireCommands []string
}

var (
	unitTests = []unitTest{
		{
			Dir:             "source",
			RequireCommands: []string{"cargo", "rustup"},
		},
	}
)

// ExecUnitTests executes all unit tests in the specified directories.
func ExecUnitTests() {
	for _, t := range unitTests {
		for _, c := range t.RequireCommands {
			if _, err := exec.LookPath(c); err != nil {
				fmt.Printf("Command %s does not exist in PATH\n", c)
				os.Exit(1)
			}
		}

		lib.PrintDiver()
		fmt.Printf("Running [%s] tests\n", t.Dir)
		lib.PrintDiver()

		srcPath := fmt.Sprintf("../%s", t.Dir)
		cargoManifestPath := fmt.Sprintf("%s/%s", srcPath, "Cargo.toml")

		lib.ExecCmd("cargo", "+nightly", "build", "--manifest-path", cargoManifestPath)
		lib.StreamCmd("cargo", "+nightly", "test", "--manifest-path", cargoManifestPath)

		lib.PrintDiver()
		fmt.Printf("Completed [%s] tests\n", t.Dir)
		lib.PrintDiver()
	}
}

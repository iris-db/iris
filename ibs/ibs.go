package main

import (
	"fmt"
	"github.com/iris-db/iris/scripts/cli"
	"github.com/iris-db/iris/scripts/script"
	"os"
)

func main() {
	opts := cli.NewOptions()
	requireStringFlagExistance(opts.Type, cli.TypeFlag)
	requireStringFlagExistance(opts.Name, cli.NameFlag)

	s, err := script.New(opts)
	if err != nil {
		if err, ok := err.(*script.Error); ok {
			err.PrettyFatalLog()
		}
		script.SetupError(err).PrettyFatalLog()
	}

	if err := s.Start(); err != nil {
		if err, ok := err.(*script.Error); ok {
			err.PrettyFatalLog()
		}
		script.RunError(err).PrettyFatalLog()
	}

	fmt.Printf("\nSuccessfuly executed the %s script.\nAny generated binaries will be in the build/%s directory.\n", s.Name, s.Name)
}

// requireStringFlagExistance requires that a string flag exists, exiting the
// programming if it does not (exit code 1).
func requireStringFlagExistance(value, name string) {
	if value != "" {
		return
	}
	fmt.Printf("Missing required flag: %v\n", name)
	os.Exit(1)
}

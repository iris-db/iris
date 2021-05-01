package main

import (
	"flag"
	"github.com/iris-db/iris/t/tests"
)

type testConfig struct {
	Unit    bool
	E2E     bool
	Cluster bool
}

func main() {
	c := readFlags()

	if c.Unit {
		tests.ExecUnitTests()
	}
}

// readFlags reads the command line arguments for tests.
func readFlags() testConfig {
	unit := flag.Bool("u", false, "Unit tests")
	e2e := flag.Bool("e", false, "End to end tests")
	cluster := flag.Bool("c", false, "Distributed cluster tests")
	all := flag.Bool("a", false, "Runs all tests")

	flag.Parse()

	if *all {
		return testConfig{
			Unit:    true,
			E2E:     true,
			Cluster: true,
		}
	}

	return testConfig{
		Unit:    *unit,
		E2E:     *e2e,
		Cluster: *cluster,
	}
}

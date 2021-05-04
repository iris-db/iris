package main

import (
	"flag"
)

type buildOpts struct {
	dist bool
}

func main() {
	//opts := readFlags()
}

func readFlags() buildOpts {
	dist := flag.Bool("image", false, "Builds a distributable docker image")

	flag.Parse()

	return buildOpts{
		dist: *dist,
	}
}

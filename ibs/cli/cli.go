package cli

import (
	"flag"
)

// Options holds the details of the YML script to execute.
type Options struct {
	Type string
	Name string
}

const (
	// TypeFlag is the flag that specifies if the script if a BUILD or TEST script.
	TypeFlag = "type"
	// NameFlag is the name of the script to look for.
	NameFlag = "name"
)

// NewOptions reads CLI flags and parses them into the Options struct.
func NewOptions() *Options {
	t := flag.String(TypeFlag, "", "Determines if you are running a BUILD or TEST script.")
	n := flag.String(NameFlag, "", "The script name.")

	flag.Parse()

	return &Options{*t, *n}
}

package script

import (
	"errors"
	"strings"
)

// Type is whether the CLI will execute a TEST or BUILD directive.
type Type int

const (
	// Build is a script that builds a binary or compiles a file.
	Build Type = iota
	// Test runs unit, integration, or end to end tests for a project.
	Test
)

// ToString converts a Type into a string.
func (t Type) ToString() string {
	return map[Type]string{
		Build: "build",
		Test:  "test",
	}[t]
}

// ModeFromString returns a Type from a string.
//
// Modes must be either TEST or BUILD (case insensitive). Returns an error if the mode is invalid.
func ModeFromString(s string) (Type, error) {
	switch strings.ToLower(s) {
	case "build":
		return Build, nil
	case "test":
		return Test, nil
	default:
		return Build, errors.New("improper type. Must be build or test")
	}
}

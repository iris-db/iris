package script

import (
	"fmt"
	"os"
	"unicode"
)

// Error is an error that occured while executing a Script.
type Error struct {
	Stage ExecutionStage
	Err   error
}

// SetupError returns an error with the setup stage.
func SetupError(err error) *Error {
	return &Error{
		Stage: SetupStage,
		Err:   err,
	}
}

// ValidationError returns an error with the validation stage.
func ValidationError(err error) *Error {
	return &Error{
		Stage: ValidationStage,
		Err:   err,
	}
}

// RunError returns an error with the run stage.
func RunError(err error) *Error {
	return &Error{
		Stage: RunStage,
		Err:   err,
	}
}

// PrettyFatalLog logs the error message prettily and exits the program with an exit code of 1.
func (e *Error) PrettyFatalLog() {
	fmt.Println(e.Error())
	os.Exit(1)
}

func (e *Error) Error() string {
	errToks := []rune(e.Err.Error())
	errToks[0] = unicode.ToUpper(errToks[0])

	return fmt.Sprintf("[%s] %s", e.Stage, string(errToks))
}

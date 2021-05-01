package lib

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
)

// RequiredCommand is a command that is required to be installed in the system PATH for the test to successfully work.
// By default, if no custom validator is present then the only check that will occur is that the command name is
// installed in the system PATH. The validator will preform additional checks to ensure that a command will
// work properly.
type RequiredCommand struct {
	cmd       string
	validator *CommandValidator
}

// CommandValidator further validates a RequiredCommand by executing custom validation logic, such as checking if
// something is installed.
type CommandValidator struct {
	Error    error                 // Error is the error thrown if the validation is not successful.
	Validate func(cmd string) bool // Validate executes the validation logic.
}

// RequiredCommandOpt is a configuration option for a command.
type RequiredCommandOpt func(*RequiredCommand)

// WithValidation adds a custom validation step to a RequiredCommand.
func WithValidation(validator CommandValidator) RequiredCommandOpt {
	return func(cmd *RequiredCommand) {
		cmd.validator = &validator
	}
}

// NewRequiredCommand creates a new command with the specified configuration.
func NewRequiredCommand(cmd string, opts ...RequiredCommandOpt) *RequiredCommand {
	c := &RequiredCommand{cmd: cmd}
	for _, opt := range opts {
		opt(c)
	}
	return c
}

// ValidationError occurs when its validation fails.
type ValidationError struct {
	Error error
}

// Validate ensures that the command is able to successfully be executed by first checking if the cmd exists in the PATH
// and then running its validator.
func (c *RequiredCommand) Validate() error {
	cmd := c.cmd
	if _, err := exec.LookPath(cmd); err != nil {
		return fmt.Errorf("%s does not exist in PATH", cmd)
	}
	if c.validator != nil && !c.validator.Validate(cmd) {
		return c.validator.Error
	}
	return nil
}

// StreamCmd executes a command and streams its STDOUT.
func StreamCmd(name string, args ...string) error {
	cmd := exec.Command(name, args...)
	cmdReader, _ := cmd.StdoutPipe()
	scanner := bufio.NewScanner(cmdReader)

	done := make(chan bool)
	go func() {
		for scanner.Scan() {
			fmt.Println(scanner.Text())
		}
		done <- true
	}()

	if err := cmd.Start(); err != nil {
		return err
	}
	<-done
	if err := cmd.Wait(); err != nil {
		return err
	}
	return nil
}

// ExecCmdStdout executes a command, returning its STDOUT.
func ExecCmdStdout(name string, args ...string) string {
	cmd := exec.Command(name, args...)
	stdout, err := cmd.Output()
	if err != nil {
		fmt.Printf("Could not get cmd stdout: %s\n", err.Error())
		os.Exit(1)
	}
	return string(stdout)
}

// ExecCmd executes a command.
func ExecCmd(name string, args ...string) {
	cmd := exec.Command(name, args...)
	if err := cmd.Run(); err != nil {
		fmt.Print(err)
		os.Exit(1)
	}
}

package subprocess

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// Subprocess is a monitored process executed by the application.
type Subprocess struct {
	running bool // running is if the process is still running.
}

// New creates a new Subprocess.
func New() *Subprocess {
	return new(Subprocess)
}

// Start starts the subprocess.
func (s *Subprocess) Start(cmdStr string) error {
	spawner, osName, err := spawnerFromOS()
	if err != nil {
		return fmt.Errorf("operating system %s not found", osName)
	}

	t := strings.Split(cmdStr, " ")

	cmd, err := spawner.CreateCommand(t[0], strings.Join(t[0:], " "))
	if err != nil {
		return err
	}

	cmd.Stderr = os.Stdout

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		return err
	}

	_ = cmd.Start()

	scanner := bufio.NewScanner(stdout)
	scanner.Split(bufio.ScanWords)
	for scanner.Scan() {
		m := scanner.Text()
		fmt.Println(m)
	}

	_ = cmd.Wait()

	return nil
}

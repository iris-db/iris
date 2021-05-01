package lib

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
)

// StreamCmd executes a command and streams its STDOUT.
func StreamCmd(name string, args ...string) {
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
		panic(err)
	}
	<-done
	if err := cmd.Wait(); err != nil {
		panic(err)
	}
}

// ExecCmd executes a command.
func ExecCmd(name string, args ...string) {
	cmd := exec.Command(name, args...)
	if err := cmd.Run(); err != nil {
		fmt.Print(err)
		os.Exit(1)
	}
}

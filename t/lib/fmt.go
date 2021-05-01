package lib

import (
	"fmt"
	"os"
)

// PrintErrMessage prints a message and exits the program with a status code of 1.
func PrintErrMessage(msg string) {
	fmt.Println(msg)
	os.Exit(1)
}

// PrintDivider prints a horizontal divider.
func PrintDivider() {
	fmt.Println("============================")
}
